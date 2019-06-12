/// This part of the crate is for interacting with the operating system's
/// local package manager when rapture fails to install a package.
/// It supports installing through apt, scoop, and brew.

use crate::platform::Platform;

/// Get the name of the expected package manager for the current platform
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


/// Install a package using the systems expected package manager
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


/// Install a package with apt
fn apt_install(name: String) -> Result<(), String> {
    match Platform::command(format!("sudo apt install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("apt failed to install {}", name))
    }
}

/// Install a package with brew
fn brew_install(name: String) -> Result<(), String> {
    match Platform::command(format!("brew install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("brew failed to install {}", name))
    }
}

/// Install a package with scoop
fn scoop_install(name: String) -> Result<(), String> {
    match Platform::command(format!("scoop install {}", name)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("scoop failed to install {}", name))
    }
}