use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use walkdir::{WalkDir, DirEntry};

/// This function encodes a vector of bytes into a string of hex characters.
/// This is used with the output of File::read_to_end to convert a file to hex.
pub fn encode(buf: Vec<u8>) -> Result<String, String> {
    Ok(*hex_d_hex::lower_hex(&buf))
}

/// This function decodes a string of hex characters into a vector of bytes.
/// These bytes can be written to a file using File::write_all.
pub fn decode(string: String) -> Result<Vec<u8>, String> {
    let hex_vals = *hex_d_hex::dhex(&string);
    Ok(hex_vals)
}

/// This function is used by the capture function to ignore hidden folders and files.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}

/// This is the heart of the capture subcommand.
/// This function walks the over the folders and files of an entire file tree,
/// and generates a rapture script that will recreate that exact file tree within
/// the package installation directory.
/// 
/// It takes two arguments: package_name, and directory.
///
/// package_name is the name of the package the output rapture script will install to.
/// Basically, it just adds `package your_package_name_here` to the top of the output script.
/// 
/// directory is the path to the directory to capture.
pub fn capture<S: Display>(package_name: S, directory: S) -> Result<(), String> {
    // Here we create the output rapture file we will write to.
    let mut output_rapture = match OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.rapt", package_name))
    {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Could not open output rapture file: {}", e.to_string())),
    }?;

    // Write the package declaration to the top of the 
    // file so we get access to the `write-hex` and `mkdir` commands.
    match writeln!(output_rapture, "package {}", package_name.to_string()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Failed to write package declaration to file")),
    }?;

    // Walk over each folder in the captured directory, and add a `mkdir` instruction
    // to the output rapture file for that directory path.
    for entry in WalkDir::new(directory.to_string())
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) // filter hidden files
        .filter_map(|e| {
            // This is really janky for now, but I can try to mess with it later.
            let result = e.ok();
            match result {
                Some(f) => {
                    if f.file_type().is_dir() {
                        Some(f)
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
    {
        // Get the folder's path in string form.
        let path = entry.path().display().to_string();

        // Format the string into a mkdir instruction
        let instruction = format!("mkdir {}", path);

        // Write the instruction to a new line in the output rapture script.
        match writeln!(output_rapture, "{}", instruction) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to append {} to file {}", instruction, path)),
        }?;
    }

    // Now that we have added a `mkdir` instruction for every folder,
    // we can create `write-hex` instructions for the files.
    // 
    // Here we dont have to ignore directories, because reading the file contents
    // will return an Err when we try to read the contents of a folder.
    for entry in WalkDir::new(directory.to_string())
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) // ignore hidden files
        .filter_map(|e| e.ok())
    {
        // Get the path to the file as a string
        let path = entry.path().display().to_string();

        // Open the file to read its contents for capturing
        let mut captured_file = File::open(path.clone());
        match &mut captured_file {
            Ok(f) => {
                // Make a buffer of bytes to read from the file
                let mut contents: Vec<u8> = vec![];

                // Read all the bytes in file to our buffer.
                match f.read_to_end(&mut contents) {
                    Ok(_) => {
                        // If there is no error reading the file, encode the bytes
                        // into a string of the hex representation of the bytes.
                        let encoded_str = encode(contents)?;

                        // Make a formatted string with the instruction to write
                        let instruction = format!("write-hex {} {}", path, encoded_str);

                        // Write the instruction onto a new line in the output rapture script file
                        match writeln!(output_rapture, "{}", instruction) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(format!("Failed to append {} to file {}", instruction, path)),
                        }?;
                    }
                    Err(_) => {}
                };
            }
            Err(_) => return Err(format!("Could not open file '{}' while capturing", path)),
        }
    }

    Ok(())
}