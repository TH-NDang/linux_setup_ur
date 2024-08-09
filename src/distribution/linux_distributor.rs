use std::{
    fmt::{Debug, Display},
    fs,
    path::PathBuf,
    process,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum DistributionType {
    Ubuntu,
    ArchLinux,
    Unknown,
}

pub trait LinuxDistributor {
    fn check() -> Self;
}

impl LinuxDistributor for DistributionType {
    fn check() -> Self {
        let arch_path: PathBuf = PathBuf::from("/etc/arch-release");
        let lsb_path: PathBuf = PathBuf::from("/etc/lsb-release");

        if arch_path.exists() {
            return DistributionType::ArchLinux;
        }

        if let Ok(content) = fs::read_to_string(lsb_path) {
            if content.contains("Ubuntu") {
                return DistributionType::Ubuntu;
            }
        }

        DistributionType::Unknown
    }
}

impl Display for DistributionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributionType::Ubuntu => write!(f, "Ubuntu"),
            DistributionType::ArchLinux => write!(f, "Arch Linux"),
            DistributionType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Identifies the Linux distribution by calling the `check` method of `DistributionType`.
pub fn identify_linux_distribution() -> DistributionType {
    DistributionType::check()
}

pub trait PackageInstaller: Debug {
    fn install_package(package: &str, use_sudo: bool) -> process::Command;
    fn remove_package(package: &str, use_sudo: bool) -> process::Command;
    fn package_manager() -> Self;
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum ArchLinux {
    #[default]
    Pacman,
    Yay,
}

impl PackageInstaller for ArchLinux {
    fn install_package(package: &str, use_sudo: bool) -> process::Command {
        let _ = use_sudo;
        let mut command: process::Command;
        match Self::package_manager() {
            ArchLinux::Pacman => {
                command = process::Command::new("pacman");
                command.arg("-S");
                command.args(["--noconfirm", "--needed"]);
                command.arg(package);
            }
            ArchLinux::Yay => {
                command = process::Command::new("yay");
                command.arg("-S");
                command.args(["--noconfirm", "--overwrite"]);
                command.arg(package);
            }
        };

        command
    }

    fn remove_package(package: &str, use_sudo: bool) -> process::Command {
        todo!()
    }

    fn package_manager() -> Self {
        let ouput = process::Command::new("yay")
            .arg("--version")
            .output()
            .expect("Failed to check for yay");

        String::from_utf8_lossy(&ouput.stdout)
            .contains("yay v")
            .then(|| ArchLinux::Yay)
            .unwrap_or(ArchLinux::Pacman)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Ubuntu {
    #[default]
    Apt,
}

impl PackageInstaller for Ubuntu {
    fn install_package(package: &str, use_sudo: bool) -> process::Command {
        let mut command: process::Command;

        if use_sudo {
            command = process::Command::new("sudo");
            command.arg("apt");
        } else {
            command = process::Command::new("apt");
        }

        command.args(["install", "-y"]);
        command.arg(package);

        command
    }

    fn remove_package(package: &str, use_sudo: bool) -> process::Command {
        todo!()
    }

    fn package_manager() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_linux_distribution() {
        assert_eq!(format!("{}", DistributionType::Ubuntu), "Ubuntu");
        assert_eq!(format!("{}", DistributionType::ArchLinux), "Arch Linux");
        assert_eq!(format!("{}", DistributionType::Unknown), "Unknown");
    }

    #[test]
    fn test_identify_linux_distribution() {
        // This test is environment-dependent and may need to be adjusted based on the actual system
        let distro = identify_linux_distribution();
        assert!(matches!(
            distro,
            DistributionType::Ubuntu | DistributionType::ArchLinux | DistributionType::Unknown
        ));
    }
}
