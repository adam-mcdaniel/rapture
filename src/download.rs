// use crate::package::Package;
use crate::platform::Platform;
use crate::path::{PathManager, path_to_string};
use crate::script::Script;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;


pub struct Downloader {}


impl Downloader {
    pub fn download_script(url: String) -> Result<Script, String> {
        PathManager::make_install_dir()?;

        let mut download_pathbuf = PathBuf::new();
        download_pathbuf.push(PathManager::install_dir());
        download_pathbuf.push("rapture_download.txt");
        let download_path = path_to_string(download_pathbuf);

        Self::download_file(url, download_path.clone())?;
        let mut downloaded_file = File::open(download_path);
        match &mut downloaded_file {
            Ok(f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                Ok(Script::new(contents))
            },
            Err(_) => Err("Could not open downloaded script".to_string())
        }
        
    }

    pub fn download_file(url: String, output_file: String) -> Result<(), String> {
        match Platform::command(format!("curl '{}' -o {}", url, output_file)) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Curl failed to download the file at {} to {}", url, output_file))
        }
    }
}