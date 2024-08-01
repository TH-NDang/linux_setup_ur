use serde::{Deserialize, Serialize};

use crate::{utils::Status, CommandRunner, Repository};

use super::CommandStruct;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRepository {
    commands: Vec<CommandStruct>,
}

impl Repository<CommandStruct> for CommandRepository {
    fn new() -> Self {
        CommandRepository {
            commands: Vec::new(),
        }
    }

    fn add(&mut self, item: CommandStruct) {
        self.commands.push(item);
    }
}

impl CommandRunner for CommandRepository {
    fn run(&self) -> Status {
        let failed = self
            .commands
            .iter()
            .filter(|command| command.run() == Status::Failure)
            .count();

        if failed > 0 {
            Status::Failure
        } else {
            Status::Success
        }
    }
}
