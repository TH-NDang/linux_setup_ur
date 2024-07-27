use serde::{Deserialize, Serialize};

use crate::{utils::Status, CommandRunner, Repository};

use super::{CommandFactory, CommandStruct};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRepository {
    commands: Vec<CommandStruct>,
}

impl From<Vec<&str>> for CommandRepository {
    fn from(value: Vec<&str>) -> Self {
        let mut repo = CommandRepository::new();
        for command in value {
            repo.add(CommandFactory::new(command));
        }
        repo
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_repository_add_and_run_commands() {
        let mut repo = CommandRepository::new();
        let command1 = CommandFactory::new("echo Command 1");
        let command2 = CommandFactory::new("echo Command 2");

        repo.add(command1);
        repo.add(command2);

        assert_eq!(repo.commands.len(), 2);

        repo.run();
    }

    #[test]
    fn test_command_repository_empty() {
        let repo = CommandRepository::new();
        assert!(repo.commands.is_empty());
    }

    #[test]
    fn test_command_repository_add_command() {
        let mut repo = CommandRepository::new();
        let command = CommandFactory::new("echo Test");
        repo.add(command);
        assert_eq!(repo.commands.len(), 1);
    }

    #[test]
    fn test_command_repository_from_vec() {
        let commands = vec!["echo Command 1", "echo Command 2"];
        let repo: CommandRepository = commands.into();
        assert_eq!(repo.commands.len(), 2);
        assert_eq!(repo.commands[0].command(), "echo Command 1");
        assert_eq!(repo.commands[1].command(), "echo Command 2");
    }
}
