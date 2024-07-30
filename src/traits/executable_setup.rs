use crate::utils::Status;

pub trait ExecutableSetup {
    fn setup(&mut self) -> Status;
}
