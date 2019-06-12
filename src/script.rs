use crate::path::{PathManager, path_to_string};
use crate::platform::Platform;
use crate::backup;
use crate::frontend::{install, gitclone, add_to_path};
use crate::capture::decode;
use std::fmt::{Display, Formatter, Error};
use std::fs::{create_dir_all, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;


#[derive(Clone)]
pub struct Script {
    package_name: Option<String>,
    script: String,
}


fn split_first_space<'a>(s: String) -> Result<(String, String), ()> {
    let mut split_index = 0;
    for (i, c) in s.clone().chars().enumerate() {
        split_index = i;
        match c {
            ' ' | '\t' | '\n' => {
                break;
            },
            _ => {}
        }
    }

    if split_index >= s.to_string().len() - 1 {
        return Err(())
    }

    let head = &s.as_str()[..split_index];
    let tail = &s.as_str()[split_index..];
    return Ok((head.trim().to_string(), tail.trim().to_string()))
}


impl Script {
    pub fn new<S: ToString>(script: S) -> Self {
        return Self {
            package_name: None,
            script: script.to_string(),
        }
    }

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


    pub fn run(&mut self) -> Result<(), String> {
        let lines = self.script.lines();

        for line in lines {
            let (command, args) = match split_first_space(line.to_string()) {
                Ok((c, a)) => (c, a),
                Err(_) => continue
            };

            match (command.as_str(), args.as_str()) {
                ("package", name) => {
                    PathManager::make_package_dir(name.to_string())?;
                    PathManager::add_to_path(name.to_string())?;
                    self.package_name = Some(name.to_string());
                },
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
                ("rapt-install", url) => {
                    install(url.to_string())?;
                },
                ("backend-install", url) => {
                    backup::install(url.to_string())?;
                },
                ("write-hex", path_hex) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            let (path, bytes) = match split_first_space(path_hex.to_string()) {
                                Ok((c, a)) => (c, a),
                                Err(_) => continue
                            };
                            
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);
                            
                            let mut file = match OpenOptions::new()
                                .create(true)
                                .write(true)
                                .open(absolute_path.clone()) 
                            {
                                Ok(f) => Ok(f),
                                Err(_) => Err(format!("Could not open file '{}'", path_to_string(absolute_path.clone())))
                            }?;

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
                ("mkdir", path) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);
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
                ("echo", string) => {
                    println!("{}", string);
                },
                ("add-path", path) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            let package_dir = PathManager::package_dir(name.to_string());
                            let mut absolute_path = PathBuf::new();
                            absolute_path.push(package_dir);
                            absolute_path.push(path);
                            add_to_path(path_to_string(absolute_path))?;
                        },
                        None => {
                            return Err("Tried to add to path without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
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
                ("*", cmd) => {
                    self.command(cmd)?;
                },
                ("#", _) => {
                    // # will denote a comment
                },
                ("", "") => {},
                (command, args) => {
                    return Err(format!("Unrecognized command '{} {}'", command, args));
                }
            }
        }
        Ok(())
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SCRIPT: {}", self.script)
    }
}
