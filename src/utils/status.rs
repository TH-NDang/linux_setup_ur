use serde::{Deserialize, Serialize};
use std::fmt;

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
        use Status::*;
        match self {
            Running => println!("{}==> ⏳ Running: {}{}", self, message, Normal),
            Success => println!("{}==> ✅ Success: {}{}", self, message, Normal),
            Warning => println!("{}==> ⚠️ Warning: {}{}", self, message, Normal),
            Failure => eprintln!("{}==> ❌ Failed: {}{}", self, message, Normal),
            Normal => println!("{}", message),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_color())
    }
}

impl Status {
    fn to_color(&self) -> Color {
        use Status::*;
        match self {
            Running => Color::Blue,
            Success => Color::Green,
            Warning => Color::Yellow,
            Failure => Color::Red,
            Normal => Color::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Status::*;

    #[test]
    fn test_display_status() {
        assert_eq!(format!("{}", Running), "\x1b[34m");
        assert_eq!(format!("{}", Success), "\x1b[32m");
        assert_eq!(format!("{}", Warning), "\x1b[33m");
        assert_eq!(format!("{}", Failure), "\x1b[31m");
        assert_eq!(format!("{}", Normal), "\x1b[0m");
    }

    #[test]
    fn test_print_message() {
        Running.print_message("Test running");
        Success.print_message("Test success");
        Warning.print_message("Test warning");
        Failure.print_message("Test failure");
        Normal.print_message("Test normal");
    }
}
