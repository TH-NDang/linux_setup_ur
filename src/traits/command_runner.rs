use crate::utils::Status;

pub trait CommandRunner {
    fn run(&self) -> Status;
}
