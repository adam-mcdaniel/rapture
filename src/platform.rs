use std::process::Command;
use std::fmt::Display;
use os_info::{get, Type};



/// This enumeration is used for detecting the user's operating system and
/// for writing things to the commandline. It automatically switches formats
/// the shell command to utilize the operating system's corresponding shell.
#[derive(PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Ubuntu,
    Unknown // Unknown is automatically assumed to be a unix like OS
}


impl Platform {
    /// This returns which operating system the user is currently using,
    /// in the form of a Platform enumeration member.
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

    /// This function writes a command the operating system's respective
    /// command line shell.
    pub fn command<S: Display>(s: S) -> Result<(), String> {
        // Trim extraneous semicolons, bash / sh doesnt like it one bit!
        let s = s.to_string().trim_end_matches(";").to_string();
        match Self::get() {
            // Run a command on CMD
            Platform::Windows => {
                match Command::new("cmd").args(&["/C", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run Windows command '{}'", s))
                }
            },

            // Run a command on bash
            Platform::MacOS => {
                match Command::new("bash").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run MacOS command '{}'", s))
                }
            },
            // Run a command on bash
            Platform::Ubuntu => {
                match Command::new("bash").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run Ubuntu command '{}'", s))
                }
            },
            // Because it's not known if `Unknown` has bash, use more widespread sh shell.
            Platform::Unknown => {
                match Command::new("sh").args(&["-c", &format!("{}", s)]).output() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to run Linux command '{}'", s))
                }
            }
        }
    }
}