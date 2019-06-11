extern crate clap;
use clap::{clap_app};
use rapture::frontend::install;

fn main() {
    let matches = clap_app!(rapture =>
            (version: "0.1.0")
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (about: "A cross platform install script library / package manager")
            (@subcommand install =>
                (about: "Install a package")
                (version: "0.0.0")
                (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
                (@arg PACKAGE: +required "The url for the package to install")
            )
    ).get_matches();


    if let Some(install_matches) = matches.subcommand_matches("install") {
        match install_matches.value_of("PACKAGE") {
            Some(package) => {
                println!("You want to install: \"{}\"", package);
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
                println!("ERR");
            }
        }
    }
}