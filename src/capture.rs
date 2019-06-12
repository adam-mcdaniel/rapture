use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use walkdir::{WalkDir, DirEntry};


pub fn encode(buf: Vec<u8>) -> Result<String, String> {
    Ok(*hex_d_hex::lower_hex(&buf))
}


pub fn decode(string: String) -> Result<Vec<u8>, String> {
    let hex_vals = *hex_d_hex::dhex(&string);
    Ok(hex_vals)
}


fn is_hidden(entry: &DirEntry) -> bool {
    let result = entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false);
    let path = entry.path().display().to_string();
    println!("{} {}", path, result);
    result
}


pub fn capture<S: Display>(package_name: S, directory: S) -> Result<(), String> {
    let mut output_rapture = match OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.rapt", package_name))
    {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Could not open output rapture file: {}", e.to_string())),
    }?;

    match writeln!(output_rapture, "package {}", package_name.to_string()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Failed to write package declaration to file")),
    }?;

    for entry in WalkDir::new(directory.to_string())
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) 
        .filter_map(|e| {
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
        let path = entry.path().display().to_string();

        let instruction = format!("mkdir {}", path);
        match writeln!(output_rapture, "{}", instruction) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to append {} to file {}", instruction, path)),
        }?;
    }

    for entry in WalkDir::new(directory.to_string())
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) 
        .filter_map(|e| e.ok())
    {
        let path = entry.path().display().to_string();

        let mut captured_file = File::open(path.clone());
        match &mut captured_file {
            Ok(f) => {
                let mut contents: Vec<u8> = vec![];

                match f.read_to_end(&mut contents) {
                    Ok(_) => {
                        let encoded_str = encode(contents)?;
                        let instruction = format!("write-hex {} {}", path, encoded_str);
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