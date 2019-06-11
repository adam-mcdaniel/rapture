use crate::platform::Platform;
use crate::frontend::install;
use std::fmt::{Display, Formatter, Error};


pub struct Script {
    script: String,
}


fn split_first_space<'a>(s: String) -> (String, String) {
    let mut split_index = 0;
    for (i, c) in s.clone().chars().enumerate() {
        split_index = i;
        match c {
            ' ' | '\t' | '\n' => {
                break;
            },
            _ => {}
        }
    }

    let head = &s.as_str()[..split_index];
    let tail = &s.as_str()[split_index..];
    return (head.trim().to_string(), tail.trim().to_string())
}


impl Script {
    pub fn new<S: ToString>(script: S) -> Self {
        return Self {
            script: script.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let lines = self.script.lines();

        for line in lines {
            let (command, args) = split_first_space(line.to_string());
            match (command.as_str(), args.as_str()) {
                ("install", url) => {
                    install(url.to_string())?;
                },
                ("WINDOWS", cmd) => {
                    if Platform::get() == Platform::Windows {
                        Platform::command(cmd)?;
                    }
                },
                ("MACOS", cmd) => {
                    if Platform::get() == Platform::MacOS {
                        Platform::command(cmd)?;
                    }
                },
                ("UBUNTU", cmd) | ("LINUX", cmd) => {
                    if Platform::get() == Platform::Ubuntu {
                        Platform::command(cmd)?;
                    }
                },
                ("UNKNOWN", cmd) => {
                    if Platform::get() == Platform::Unknown {
                        Platform::command(cmd)?;
                    }
                },
                ("*", cmd) => {
                    Platform::command(cmd)?;
                },
                (command, args) => {
                    return Err(format!("Unrecognized command '{} {}'", command, args));
                }
            }
        }
        Ok(())
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "SCRIPT: {}", self.script)
    }
}
