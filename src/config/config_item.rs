use serde::{Deserialize, Serialize};

use crate::{traits::ProcessRunner, utils::Status, CommandStruct, Configurator};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    commands: Vec<CommandStruct>,
}

impl Configurator for Config {
    fn apply(&self) -> Status {
        Status::Running.print_message("Applying configuration");
        let failed = self
            .commands
            .iter()
            .filter(|command| command.execute() == Status::Failure)
            .count();

        if failed > 0 {
            return Status::Failure;
        }

        Status::Success
    }

    fn revert(&self) -> Status {
        todo!()
    }
}
