use std::{fs, path};


/// Checks if a file exists at the specified path.
///
/// ### Arguments
///
/// * `path` - A string slice that holds the path to the file.
///
/// ### Returns
///
/// A boolean value indicating whether the file exists or not.
pub(crate) fn file_exists(path: &str) -> bool {
    path::Path::new(path).exists()
}

/// Reads the content of a file specified by the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// A `Result` containing a `String` with the content of the file if successful, or an `std::io::Error` if an error occurs.
pub(crate) fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exists() {
        assert!(file_exists("/etc/passwd"));
        assert!(!file_exists("/nonexistent-file"));
    }

    #[test]
    fn test_read_file_content() {
        let content = read_file_content("/etc/passwd");
        assert!(content.is_ok());
        let content = read_file_content("/nonexistent-file");
        assert!(content.is_err());
    }
}
