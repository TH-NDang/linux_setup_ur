use serde::{Deserialize, Serialize};

use crate::{utils::Status, CommandFactory, CommandRunner, CommandStruct, ConfigItem};
use crate::Configurator;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupEntry {
    check: Option<String>,
    commands: Vec<CommandStruct>,
    configs: Option<Vec<ConfigItem>>,
}

impl From<(Option<&str>, Vec<&str>, Option<Vec<&str>>)> for SetupEntry {
    fn from(value: (Option<&str>, Vec<&str>, Option<Vec<&str>>)) -> Self {
        let mut setup = SetupEntry::new();

        if let Some(check) = value.0 {
            setup.set_check(check);
        }

        for command in value.1 {
            setup.commands.push(CommandFactory::new(command));
        }
        let mut configs = Vec::new();

        if let Some(configs_list) = value.2 {
            for config in configs_list {
                configs.push(ConfigItem {
                    check: None,
                    command: CommandFactory::new(config),
                });
            }
        }

        setup.configs = Some(configs);
        setup
    }
}

impl SetupEntry {
    pub fn new() -> Self {
        SetupEntry {
            check: None,
            commands: Vec::new(),
            configs: None,
        }
    }

    pub fn set_check(&mut self, check: &str) {
        self.check = Some(check.to_string());
    }

    fn check(&self) -> bool {
        if let Some(check_command) = &self.check {
            return CommandStruct::check_command_success(&check_command, |output| {
                !String::from_utf8_lossy(&output.stdout).is_empty()
            });
        }

        false
    }

    fn settings(&self) -> Status {
        if let Status::Success = self.run_commands() {
            println!("==> Running commands [config]");
            self.run_configs()
        } else {
            Status::Failure
        }
    }

    fn run_commands(&self) -> Status {
        let failed = self
            .commands
            .iter()
            .filter(|command| command.run() == Status::Failure)
            .count();

        if failed > 0 {
            return Status::Failure;
        }

        Status::Success
    }

    fn run_configs(&self) -> Status {
        if let Some(configs) = &self.configs {
            let failed = configs
                .iter()
                .filter(|config| config.apply() == Status::Failure)
                .count();

            if failed > 0 {
                return Status::Failure;
            }
        }

        Status::Success
    }
    
    pub fn commands(&self) -> &[CommandStruct] {
        &self.commands
    }
}

impl CommandRunner for SetupEntry {
    fn run(&self) -> Status {
        let check = self.check();

        if !check {
            return self.settings();
        }

        Status::Success
    }
}
