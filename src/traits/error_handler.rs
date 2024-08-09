use std::io;

pub trait ErrorHandler {
    fn handle_command_error(stderr: &str) -> io::Error;
}
