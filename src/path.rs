/// For platform detection / writing to the commandline
use crate::platform::Platform;
/// Path manipulations
use std::path::PathBuf;
/// For reading/writing files and creating directories
use std::fs::{OpenOptions, create_dir_all};
/// We need some basic traits in the IO prelude for File manipulation
use std::io::prelude::*;

/// The name of the overall install directory where all packages are located.
/// This affects everything in the PathManager impl.
const INSTALL_FOLDER_NAME: &str = ".rapture";

/// Convert a pathbuf to a string
pub fn path_to_string(p: PathBuf) -> String {
    match p.into_os_string().into_string() {
        Ok(s) => s,
        Err(_) => "".to_string()
    }
}

/// An empty struct with an impl used for path manipulation
pub struct PathManager {}

impl PathManager {
    /// This function returns the users home directory. This is used by most functions
    /// in this impl to determine where everything should be installed.
    pub fn home_dir() -> String {
        match dirs::home_dir() {
            Some(dir) => path_to_string(dir),
            None => "/".to_string()
        }
    }

    /// This function returns the path to the overall install directory: the
    /// directory where all the packages will be installed. This is mainly used
    /// by make_install_dir to figure out where to put `.rapture`.
    pub fn install_dir() -> String {
        let home = Self::home_dir();
        let mut install_dir = PathBuf::new();
        install_dir.push(home);
        install_dir.push(INSTALL_FOLDER_NAME);
        return path_to_string(install_dir);
    }

    /// This function creates the install directory for all rapture packages.
    /// When rapture starts, it runs this method to verify that the `.rapture`
    /// folder exists so you're able to install packages to that directory.
    /// 
    /// This function must always be run on startup.
    pub fn make_install_dir() -> Result<(), String> {
        // the string for the absolute path to install dir
        let install_dir = Self::install_dir();
    
        // create install dir and parent dirs
        match create_dir_all(install_dir) {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }

    /// This function returns the absolute path to the hypothetical package
    /// directory with a given name. This is used by make_package_dir to 
    /// figure out where to create the package installation directory.
    pub fn package_dir(name: String) -> String {
        let install_dir = Self::install_dir();
        let mut package_dir = PathBuf::new();
        package_dir.push(install_dir);
        package_dir.push(name);
        return path_to_string(package_dir);
    }


    /// This function makes the installation directory for a specific package,
    /// as opposed to make_install_dir, which creates the directory containing
    /// all rapture's installs.
    pub fn make_package_dir(name: String) -> Result<(), String> {
        let package_dir = Self::package_dir(name);

        // Always return Ok, create_dir_all fails when the dir already exists
        match create_dir_all(package_dir) {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }

    /// This function adds a path from inside the package installation directory.
    /// For example, if i call this function with "one/two/three/four",
    /// in a package named `wonderful`, this will add 
    /// `/home/user/.rapture/wonderful/one/two/three/four` to the path.
    /// If i call this with `/one/two/three/four`, however, it will add
    /// `/one/two/three/four` to the path.
    /// 
    /// This is very experimental on windows. This is not guaranteed to work on windows
    /// platforms at all yet. Full support is intended in the future, however.
    pub fn add_to_path(name: String) -> Result<(), String> {
        // The home directory
        let home = Self::home_dir();
        match Platform::get() {
            // On linux / macos, edit .bashrc
            Platform::Unknown | Platform::MacOS | Platform::Ubuntu => {
                // Start opening the .bashrc file
                let mut bashrc = PathBuf::new();
                bashrc.push(home);
                bashrc.push(".bashrc");
                let mut file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(bashrc.clone());
                // Finish opening the .bashrc file

                // Verify file is good
                match &mut file {
                    Ok(f) => {
                        // If file is good, get the absolute path of the directory
                        // we will add the the users path in .bashrc
                        let package_dir = Self::package_dir(name);
                        // Make a fmt string to write to the file
                        let path_addition = format!("export PATH=\"$PATH:{}\"", package_dir);

                        // Read bashrc to a string
                        let mut contents = String::new();
                        f.read_to_string(&mut contents).unwrap();

                        // If bashrc contains the line already, return and do nothing
                        for line in contents.lines() {
                            if line.to_string().contains(&path_addition) {
                                return Ok(());
                            }
                        }
                        
                        // If bashrc doesnt have the path already, write it.
                        match writeln!(f, "{}", path_addition) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(format!("Failed to append to file {}", path_to_string(bashrc)))
                        }
                    },
                    Err(_) => Err(format!("Failed to add {} to PATH", name))
                }
            },

            // Windows support is very minimal.
            // I dont know if this works, but set path using the set command?
            Platform::Windows => {
                let package_dir = Self::package_dir(name);
                let path_addition = format!("set PATH=%PATH%;{}", package_dir.clone());

                // Because i dont know if this works, print the path so the user can
                // add the path if it is necessary.
                println!("WARNING: modifying the path on windows is experimental. If this install does not seem to work, add '{}' to your path environment variable.", package_dir);
                Platform::command(format!("{}", path_addition))
            },
        }
    }
}
