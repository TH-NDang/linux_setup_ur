use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::utils::Color;

/// Defines an enum representing different statuses of a command execution.
/// Implements `print_message(message: &str)` methods to print messages based on the command status.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Status {
    Running,
    Success,
    Warning,
    Failure,
    Normal,
}

impl Status {
    pub fn print_message(&self, message: &str) {
        match self {
            Status::Running => println!(
                "{}==> ⏳Running: {}{}",
                Status::Running,
                message,
                Status::Normal
            ),
            Status::Success => println!(
                "{}==> ✅Succeeded: {}{}",
                Status::Success,
                message,
                Status::Normal
            ),
            Status::Warning => println!(
                "{}==> ⚠️Warning: {}{}",
                Status::Warning,
                message,
                Status::Normal
            ),
            Status::Failure => eprintln!(
                "{}==> ❌Failed: {}{}",
                Status::Failure,
                message,
                Status::Normal
            ),
            Status::Normal => println!("{}", message),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Running => write!(f, "{}", Color::Blue),
            Status::Success => write!(f, "{}", Color::Green),
            Status::Warning => write!(f, "{}", Color::Yellow),
            Status::Failure => write!(f, "{}", Color::Red),
            Status::Normal => write!(f, "{}", Color::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_status() {
        assert_eq!(format!("{}", Status::Running), "\x1b[34m");
        assert_eq!(format!("{}", Status::Success), "\x1b[32m");
        assert_eq!(format!("{}", Status::Warning), "\x1b[33m");
        assert_eq!(format!("{}", Status::Failure), "\x1b[31m");
        assert_eq!(format!("{}", Status::Normal), "\x1b[0m");
    }
}
