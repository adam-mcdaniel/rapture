use std::process::Command;
use std::fmt::Display;
use os_info::{get, Type};



/// This enumeration is used for detecting the user's operating system and
/// for writing things to the commandline. It automatically switches formats
/// the shell command to utilize the operating system's corresponding shell.
/// 
/// NOTE: All unknown operating systems are assumed to be unix like / linux
#[derive(PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Ubuntu,
    Unknown
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
    /// 
    /// For now, there is a bug with error detection. Because Im calling
    /// bash / sh with a string containing the script instruction to run, 
    /// the stderr doesnt get processed properly by std::process::Command.
    /// 
    /// I will try to fix this bug in the future because its causing the program
    /// to continue as if its succeeding, even if its hit a catastrophic failure.
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