use std::path::PathBuf;
use std::fs::create_dir;

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
}
