use std::process::Command;
use std::fmt::Display;
use os_info::{get, Type};

#[derive(PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Ubuntu,
    Unknown
}


impl Platform {
    pub fn get() -> Self {
        let os = get();
        match os.os_type() {
            Type::Windows => Platform::Windows,
            Type::Macos => Platform::MacOS,
            Type::Ubuntu => Platform::Ubuntu,
            Type::Debian => Platform::Ubuntu,
            _ => Platform::Unknown
        }
    }

    pub fn command<S: Display>(s: S) -> Result<(), String> {
        match Self::get() {
            Platform::Windows => {
                match Command::new("cmd").args(&["/C", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run windows command '{}'", s))
                }
            },
            Platform::MacOS => {
                match Command::new("sh").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run macos command '{}'", s))
                }
            },
            Platform::Ubuntu => {
                match Command::new("sh").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run Ubuntu command '{}'", s))
                }
            },
            Platform::Unknown => {
                match Command::new("sh").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run Linux command '{}'", s))
                }
            }
        }
    }
}