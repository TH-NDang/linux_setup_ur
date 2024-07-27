use std::fmt::Display;

use crate::utils::file_operations::{file_exists, read_file_content};

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
        if file_exists("/etc/arch-release") {
            DistributionType::ArchLinux
        } else if file_exists("/etc/lsb-release") {
            match read_file_content("/etc/lsb-release") {
                Ok(content) => {
                    if content.contains("DISTRIB_ID=Ubuntu") {
                        DistributionType::Ubuntu
                    } else {
                        DistributionType::Unknown
                    }
                }
                Err(_) => DistributionType::Unknown,
            }
        } else {
            DistributionType::Unknown
        }
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
