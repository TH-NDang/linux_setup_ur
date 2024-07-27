use std::fmt;

/// Implements the `Display` trait for the Color enum, allowing custom formatting of Color values.
pub enum Color {
    Yellow,
    Green,
    Red,
    Blue,
    None,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Yellow => write!(f, "\x1b[33m"),
            Color::Green => write!(f, "\x1b[32m"),
            Color::Red => write!(f, "\x1b[31m"),
            Color::Blue => write!(f, "\x1b[34m"),
            Color::None => write!(f, "\x1b[0m"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_color() {
        assert_eq!(format!("{}", Color::Yellow), "\x1b[33m");
        assert_eq!(format!("{}", Color::Green), "\x1b[32m");
        assert_eq!(format!("{}", Color::Red), "\x1b[31m");
        assert_eq!(format!("{}", Color::Blue), "\x1b[34m");
        assert_eq!(format!("{}", Color::None), "\x1b[0m");
    }
}
