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
        use Color::*;
        match self {
            Yellow => write!(f, "\x1b[33m"),
            Green => write!(f, "\x1b[32m"),
            Red => write!(f, "\x1b[31m"),
            Blue => write!(f, "\x1b[34m"),
            None => write!(f, "\x1b[0m"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_color() {
        use Color::*;
        assert_eq!(format!("{}", Yellow), "\x1b[33m");
        assert_eq!(format!("{}", Green), "\x1b[32m");
        assert_eq!(format!("{}", Red), "\x1b[31m");
        assert_eq!(format!("{}", Blue), "\x1b[34m");
        assert_eq!(format!("{}", None), "\x1b[0m");
    }
}
