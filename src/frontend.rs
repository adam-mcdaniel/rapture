use crate::platform::Platform;
use crate::path::PathManager;
use crate::download::Downloader;
use crate::script::Script;
use crate::input::input;


pub fn install(url: String) -> Result<(), String> {
    if let Ok(mut script) = Downloader::download_script(url.clone()) {
        println!("Found rapture script '{}'", url);
        script.run()?;
        Ok(())
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