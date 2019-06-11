use std::process::Command;
use crate::platform::Platform;


pub fn installer_name() -> String {
    match Platform::get() {
        Platform::Ubuntu => {
            "apt".to_string()
        },
        Platform::MacOS => {
            "brew".to_string()
        },
        Platform::Windows => {
            "scoop".to_string()
        },
        Platform::Unknown => {
            "apt".to_string()
        }
    }
}

pub fn install(name: String) -> Result<(), String> {
    match Platform::get() {
        Platform::Ubuntu => {
            apt_install(name)
        },
        Platform::MacOS => {
            brew_install(name)
        },
        Platform::Windows => {
            scoop_install(name)
        },
        Platform::Unknown => {
            apt_install(name)
        },
    }
}


fn apt_install(name: String) -> Result<(), String> {
    match Platform::command(format!("sudo apt install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("apt failed to install {}", name))
    }
}

fn brew_install(name: String) -> Result<(), String> {
    match Platform::command(format!("brew install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("brew failed to install {}", name))
    }
}

fn scoop_install(name: String) -> Result<(), String> {
    match Platform::command(format!("scoop install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("scoop failed to install {}", name))
    }
}