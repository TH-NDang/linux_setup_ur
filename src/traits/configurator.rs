use crate::utils::Status;

pub trait Configurator {
    fn apply(&self) -> Status;
    fn revert(&self) -> Status;
}
