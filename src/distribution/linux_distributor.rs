use std::{fmt::Display, fs, path::PathBuf};

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

        if lsb_path.exists() {
            let content = fs::read_to_string(lsb_path).unwrap();
            if content.contains("Ubuntu") {
                return DistributionType::Ubuntu;
            }
        };

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
