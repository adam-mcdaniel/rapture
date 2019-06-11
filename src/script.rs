use crate::path::PathManager;
use crate::platform::Platform;
use crate::frontend::{install, gitclone, add_to_path};
use std::fmt::{Display, Formatter, Error};


pub struct Script {
    package_name: Option<String>,
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
            package_name: None,
            script: script.to_string(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let lines = self.script.lines();

        for line in lines {
            let (command, args) = split_first_space(line.to_string());
            match (command.as_str(), args.as_str()) {
                ("package", name) => {
                    PathManager::make_package_dir(name.to_string())?;
                    self.package_name = Some(name.to_string());
                },
                ("git-clone", url) => {
                    match self.package_name.clone() {
                        Some(name) => {
                            gitclone(name, url.trim_start_matches("\"")
                                                .trim_start_matches("'")
                                                .trim_end_matches("\"")
                                                .trim_end_matches("'")
                                                .to_string())?;
                        },
                        None => {
                            return Err("Tried to clone repository into package install directory without declaring the install script as a package installer via the `package PACKAGE_NAME` rapture command.".to_string())
                        }
                    }
                },
                ("install", url) => {
                    install(url.to_string())?;
                },
                ("path-add", path) => {
                    add_to_path(path.to_string())?;
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
                ("", "") => {},
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
