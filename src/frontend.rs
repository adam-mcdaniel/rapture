use crate::platform::Platform;
use crate::path::PathManager;
use crate::download::Downloader;
use crate::script::Script;
use crate::backup;
use crate::input::{input, yes_or_no};


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


pub fn gitclone(package_name: String, url: String) -> Result<(), String> {
    let package_dir = PathManager::package_dir(package_name);
    match Platform::command(format!("cd {}; git clone {}", package_dir, url)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not clone git repo '{}'", url))
    }
}


pub fn add_to_path(path: String) -> Result<(), String> {
    PathManager::add_to_path(path)
}