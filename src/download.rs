use crate::platform::Platform;
use crate::path::{PathManager, path_to_string};
use crate::script::Script;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;


/// This is an empty struct similar to PathManager, but exclusive to downloading
pub struct Downloader {}

/// This impl handles the downloading of various files. Right now we only support
/// downloading scripts from the web, but in the future I want to expand this to
/// repositories and such.
impl Downloader {
    /// This function takes a url to a rapture script, and calls `download_file` 
    /// with the url and the proper download file output location. After 
    /// downloading, we read the script from the output file and return it.
    /// 
    /// See script::Script for more information on how the script object works.
    pub fn download_script(url: String) -> Result<Script, String> {
        // Get the install directory, `~/.rapture/`
        PathManager::make_install_dir()?;

        // Here we push `rapture_download.txt` onto the path,
        // so `download_file` will download to `~/.rapture/rapture_download.txt`.
        // I chose this file location because it doesnt get in the way of packages,
        // but it stays in the rapture install directory.
        let mut download_pathbuf = PathBuf::new();
        download_pathbuf.push(PathManager::install_dir());
        download_pathbuf.push("rapture_download.txt");
        let download_path = path_to_string(download_pathbuf);

        // Download the file
        Self::download_file(url, download_path.clone())?;

        // Open the file for reading, we want to create a script object
        // with the contents of the downloaded script.
        let mut downloaded_file = File::open(download_path);
        match &mut downloaded_file {
            Ok(f) => {
                // Success!
                // Read the file as valid UTF-8
                let mut contents = String::new();
                f.read_to_string(&mut contents).unwrap();
                Ok(Script::new(contents))
            },
            // Reading the file returned an error
            Err(_) => Err("Could not open downloaded script".to_string())
        }
    }

    /// This function handles the actual legwork of downloading a file from the internet.
    /// It takes the url to the file on the internet and the path to the resulting output file.
    /// To download a file you must have curl installed and in your path!
    /// In the future, this wont depend on curl.
    pub fn download_file(url: String, output_file: String) -> Result<(), String> {
        // For now, we're using curl.
        // I'd like to make this less dependant on the OS,
        // so in the future this will be an actually HTTP request or something.
        match Platform::command(format!("curl '{}' -o {}", url, output_file)) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Curl failed to download the file at {} to {}", url, output_file))
        }
    }
}