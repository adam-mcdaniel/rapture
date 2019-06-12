/// This part of the crate is intended to be a library for other crates.
/// It supports installing a script from a url, cloning a git repository,
/// and adding to the user's path
use crate::platform::Platform;
use crate::path::PathManager;
use crate::download::Downloader;
use crate::backup;
use crate::input::{input, yes_or_no};

/// Downloads a script from the given url and runs it.
/// If there was an error running the install script, 
/// Ask the user if they want to install using their native
/// package manager.
pub fn install(url: String) -> Result<(), String> {
    if let Ok(mut script) = Downloader::download_script(url.clone()) {
        println!("Installing rapture script at '{}'", url);
        match script.run() {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("There was a problem installing the package: {}", e);
                let new_manager = backup::installer_name();
                if yes_or_no(format!("Do you want to try to install your package using {}? (y/n) ", new_manager)) {
                    let package_name = input(format!("What's the name of the package you want to install with {}? ", new_manager));
                    backup::install(package_name)?;
                }
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

/// Clone a git repository into the installation directory for the package.
pub fn gitclone(package_name: String, url: String) -> Result<(), String> {
    let package_dir = PathManager::package_dir(package_name);
    match Platform::command(format!("cd {}; git clone {}", package_dir, url)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not clone git repo '{}'", url))
    }
}

/// Add a path to the users path.
/// This mainly acts a frontend to PathManager::add_to_path.
pub fn add_to_path(path: String) -> Result<(), String> {
    PathManager::add_to_path(path)
}