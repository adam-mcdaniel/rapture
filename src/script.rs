use crate::path::{PathManager, path_to_string};
use crate::platform::Platform;
use crate::backup;
use crate::frontend::{install, gitclone, add_to_path};
use crate::capture::decode;
use std::fmt::{Display, Formatter, Error};
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

/// This object represents the core of the installation process.
/// In the future, I would like to make the scripts more complex.
/// Right now they're just a single instruction per line, and there
/// is very little to work with when writing a rapture script.
/// 
/// If I can implement an embedded language similar to but simpler than
/// lua, that would be helpful.
#[derive(Clone)]
pub struct Script {
    // This field determines whether or not the script has been declared
    // a package installer.
    // 
    // A package installer script creates a package installation directory in
    // the `~/.rapture` installation directory, while a non package installer
    // script does not.
    package_name: Option<String>,

    // Contains the contents of the script
    script: String,
}

/// This function splits a string into two pieces at the first
/// instance of whitespace. If there is no white space in the string,
/// The function returns Err(()).
fn split_first_space<'a>(s: String) -> Result<(String, String), ()> {
    let mut split_index = 0;
    // Iterate over characters in the string and update split_index
    for (i, c) in s.clone().chars().enumerate() {
        split_index = i;

        // If the character is whitespace, break
        match c {
            ' ' | '\t' | '\n' => break,
            _ => {}
        }
    }

    // If split_index is the last index in the string, return Err(())
    if split_index >= s.to_string().len() - 1 {
        return Err(())
    }
    // Otherwise, return the first and second half from the split index
    let head = &s.as_str()[..split_index];
    let tail = &s.as_str()[split_index..];
    return Ok((head.trim().to_string(), tail.trim().to_string()))
}

/// This object represents an executable rapture script
impl Script {
    /// This instantiates a new script with `script` as the script contents
    pub fn new<S: ToString>(script: S) -> Self {
        return Self {
            package_name: None,
            script: script.to_string(),
        }
    }

    /// Runs a given command on the proper operating system's shell.
    /// If the script has a package declaration, call the command
    /// within the package installation directory.
    /// 
    /// For example: if the package is named `wonderful`, and I call
    /// `cat main.rs`, the following command will be run instead:
    /// `cd /home/USERNAME/.rapture/wonderful; cat main.rs`
    /// 
    /// If I dont declare the package name before running a command,
    /// the exact command you gave this function is executed instead.
    pub fn command<S: Display>(&self, cmd: S) -> Result<(), String> {
        match self.package_name.clone() {
            Some(name) => {
                Platform::command(format!("cd {}; {};", PathManager::package_dir(name.to_string()), cmd))?;
            },
            None => {
                Platform::command(cmd)?;
            }
        }
        Ok(())
    }

    /// This function runs the rapture script. It iterates over the lines in the
    /// script, and matches the commands and the arguments. I would like to change
    /// this in the future, replacing it with an embeddable scripting language.
    pub fn run(&mut self) -> Result<(), String> {
        // The iterator for the lines in the script.
        let lines = self.script.lines();

        for line in lines {
            // Split each line by the whitespace.
            // The first string before the whitespace will be the command,
            // and the second string will be the argument.
            // 
            // If split_first_space cant find a space, it will return Err,
            // and this will skip that line and try to parse the next.
            let (command, args) = match split_first_space(line.to_string()) {
                Ok((c, a)) => (c, a),
                Err(_) => continue
            };

            // Match the command and args as a tuple of &str
            match (command.as_str(), args.as_str()) {
                // The current instruction is a package declaration.
                // First, we create the directory where the package contents
                // will be installed.
                // 
                // Then we add the directory to the user's path,
                // and give the running script the package name to use in future commands.
                ("package", name) => {
                    PathManager::make_package_dir(name.to_string())?;
                    PathManager::add_to_path(name.to_string())?;
                    self.package_name = Some(name.to_string());
                },
                // Clone a git repository into the current package.
                // If the current script is not a package installer, throw an error.
                ("git-clone", url) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            gitclone(name, url.trim_start_matches("\"")
                                                .trim_start_matches("'")
                                                .trim_end_matches("\"")
                                                .trim_end_matches("'")
                                                .to_string())?;
                        },
                        None => {
                            return Err("Tried to clone repository into package install directory without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
                // Download a rapture script from url and install it before continuing.
                ("rapt-install", url) => {
                    install(url.to_string())?;
                },
                // Call the operating system's native package manager.
                ("backend-install", package) => {
                    backup::install(package.to_string())?;
                },
                // This is mainly a feature of the `capture` subcommand.
                // This is not meant for users to be messing around with.
                ("write-hex", path_hex) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            // Split the argments into a path and the bytes to write
                            let (path, bytes) = match split_first_space(path_hex.to_string()) {
                                Ok((c, a)) => (c, a),
                                Err(_) => continue
                            };
                            
                            // Get the path relative to the package install directory
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);
                            
                            // Open the file for writing
                            let mut file = match OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(absolute_path.clone()) 
                            {
                                Ok(f) => Ok(f),
                                Err(_) => Err(format!("Could not open file '{}'", path_to_string(absolute_path.clone())))
                            }?;

                            // Decode the hex string into a list of Vec<u8>.
                            // These are not UTF-8 characters!!! These are the 
                            // bytes to write directly to the opened file.
                            match decode(bytes.clone()) {
                                Ok(vector) => match file.write_all(&vector) {
                                    Ok(_) => {},
                                    Err(_) => return Err(format!("Could not write decoded bytes to file '{}'", path_to_string(absolute_path)))
                                },
                                Err(_) => return Err(format!("Could not decode hex code '{}'", bytes.clone()))
                            };
                        },
                        None => {
                            return Err("Tried to write hex to a file without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
                // Make a directory.
                // This can be a directory that has non-existant parent directories.
                // For example, if I invoke the rapture command:
                // `mkdir ./cmake/contrib/profiling`
                // Rapture will create each of the parent directories if they do not already exist.
                ("mkdir", path) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            // Make the path a relative path to the package install directory
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);

                            // Create the folder using create_dir_all.
                            // create_dir_all creates parent directories as needed,
                            // similar to mkdir -p DIRECTORY
                            match create_dir_all(absolute_path.clone()) {
                                Ok(()) => {},
                                Err(_) => return Err(format!("Failed to create directory {}", path_to_string(absolute_path)))
                            }
                        },
                        None => {
                            return Err("Tried to make directory without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
                // This prints a message to the console
                ("echo", string) => {
                    println!("{}", string);
                },
                // This command adds a path to the users path.
                // This is mainly used if there is a `bin` directory or another directory
                // within the package install directory that needs to be added to the path.
                // 
                // add-path can only be used after the package declaration.
                ("add-path", path) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            // Get the path relative to the package dir
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);

                            // Call frontend::add_to_path
                            add_to_path(path_to_string(absolute_path))?;
                        },
                        None => {
                            return Err("Tried to add to path without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
                // The hastag symbol denotes a comment
                ("#", _) => {},

                // The following commands run `arg` as a shell
                // command on their respective operating systems.
                ("WINDOWS", cmd) => {
                    if Platform::get() == Platform::Windows {
                        self.command(cmd)?;
                    }
                },
                ("MACOS", cmd) => {
                    if Platform::get() == Platform::MacOS {
                        self.command(cmd)?;
                    }
                },
                ("UBUNTU", cmd) | ("LINUX", cmd) => {
                    if Platform::get() == Platform::Ubuntu {
                        self.command(cmd)?;
                    }
                },
                ("UNKNOWN", cmd) => {
                    if Platform::get() == Platform::Unknown {
                        self.command(cmd)?;
                    }
                },
                // Runs a command on all operating systems
                ("*", cmd) => {
                    self.command(cmd)?;
                },
                ("", "") => {},
                // An unrecognized command was given, return Err
                (command, args) => {
                    return Err(format!("Unrecognized command '{} {}'", command, args));
                }
            }
        }
        Ok(())
    }
}

// Dummy Display impl used for debugging
impl Display for Script {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.script)
    }
}
