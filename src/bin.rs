use clap::{clap_app, crate_version};
use rapture::frontend::install;
use rapture::script::Script;
use rapture::capture::capture;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let matches = clap_app!(rapture =>
            (version: crate_version!())
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (about: "A cross platform install script library / package manager")
            (@subcommand install =>
                (about: "Install a rapture package")
                (version: "0.0.1")
                (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                (@arg INPUT_FILE: -f --file +takes_value "Install from an input rapture file")
                (@arg PACKAGE: "The url for the package to install")
            )
            (@subcommand capture =>
                (about: "Capture a directory and package it into a rapture package")
                (version: "0.0.1")
                (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                (@arg PACKAGE_NAME: +required "The name of the generated package")
                (@arg DIRECTORY: +required "The path to the directory to capture")
            )
    ).get_matches();


    if let Some(install_matches) = matches.subcommand_matches("install") {
        match install_matches.value_of("PACKAGE") {
            Some(package) => {
                match install(package.to_string()) {
                    Ok(()) => {
                        println!("Successfully installed package.")
                    },
                    Err(e) => {
                        println!("There was a problem installing the package: {}", e);
                    }
                }
            },
            None => {
                match install_matches.value_of("INPUT_FILE") {
                    Some(file) => {
                        let mut rapture_file = File::open(file);
                        match &mut rapture_file {
                            Ok(f) => {
                                let mut contents = String::new();
                                f.read_to_string(&mut contents).unwrap();
                                match Script::new(contents).run() {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("There was a problem installing the package: {}", e);
                                    }
                                };
                                println!("Successfully installed package.")
                            },
                            Err(_) => println!("Could not open rapture script")
                        }
                    },
                    None => {
                        println!("No script or url provided");
                    }
                }
            }
        }
    }


    if let Some(capture_matches) = matches.subcommand_matches("capture") {
        let package_name = capture_matches.value_of("PACKAGE_NAME").unwrap();
        let directory = capture_matches.value_of("DIRECTORY").unwrap();
        match capture(package_name, directory) {
            Ok(()) => {
                println!("Successfully captured directory");
            },
            Err(e) => {
                println!("There was a problem installing the package: {}", e);
            }
        };
    }
}