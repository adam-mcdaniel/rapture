use crate::platform::Platform;
use std::path::PathBuf;
use std::fs::{OpenOptions, create_dir};
use std::io::prelude::*;

const INSTALL_FOLDER_NAME: &str = ".rapture";


pub fn path_to_string(p: PathBuf) -> String {
    p.into_os_string().into_string().unwrap()
}

pub struct PathManager {}

impl PathManager {
    pub fn home_dir() -> String {
        match dirs::home_dir() {
            Some(dir) => path_to_string(dir),
            None => "/".to_string()
        }
    }

    pub fn install_dir() -> String {
        let home = Self::home_dir();
        let mut install_dir = PathBuf::new();
        install_dir.push(home);
        install_dir.push(INSTALL_FOLDER_NAME);
        return path_to_string(install_dir);
    }

    pub fn make_install_dir() -> Result<(), String> {
        let install_dir = Self::install_dir();
        
        match create_dir(install_dir) {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
            // Err(_) => Err("Could not make rapture install directory".to_string())
        }
    }

    pub fn package_dir(name: String) -> String {
        let install_dir = Self::install_dir();
        let mut package_dir = PathBuf::new();
        package_dir.push(install_dir);
        package_dir.push(name);
        return path_to_string(package_dir);
    }

    pub fn make_package_dir(name: String) -> Result<(), String> {
        let package_dir = Self::package_dir(name);
        
        match create_dir(package_dir) {
            Ok(_) => Ok(()),
            Err(_) => Ok(())
        }
    }

    pub fn add_to_path(name: String) -> Result<(), String> {
        let home = Self::home_dir();
        match Platform::get() {
            Platform::Unknown | Platform::MacOS | Platform::Ubuntu => {
                let mut bashrc = PathBuf::new();
                bashrc.push(home);
                bashrc.push(".bashrc");
                let mut file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(bashrc.clone());
                match &mut file {
                    Ok(f) => {
                        let package_dir = Self::package_dir(name);
                        let path_addition = format!("export PATH=\"$PATH:{}\"", package_dir);

                        let mut contents = String::new();
                        f.read_to_string(&mut contents).unwrap();

                        for line in contents.lines() {
                            if line.to_string().contains(&path_addition) {
                                return Ok(());
                            }
                        }
                        
                        match writeln!(f, "{}", path_addition) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(format!("Failed to append to file {}", path_to_string(bashrc)))
                        }


                    },
                    Err(_) => Err(format!("Failed to add {} to PATH", name))
                }
            },
            Platform::Windows => {
                Err(format!("Adding to path is not yet supported for Windows"))
            },
        }
    }
}
