use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::traits::executable_setup::ExecutableSetup;
use crate::Configurator;
use crate::{utils::Status, CommandRunner, CommandStruct, ConfigItem};

#[derive(Serialize, Deserialize, Debug)]
struct SetupItem {
    env_vars: Option<Vec<String>>,
    working_dir: Option<PathBuf>,
}

impl SetupItem {
    fn ensure_working_dir(&self) -> io::Result<()> {
        if let Some(dir) = &self.working_dir {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
                println!("Created directory: {:?}", dir);
            }
        }
        Ok(())
    }

    fn ensure_env_vars(&mut self) -> io::Result<()> {
        if let Some(vars) = &mut self.env_vars {
            for env_var in vars.iter() {
                if std::env::var(env_var).is_err() {
                    println!("Environment variable `{}` not set.", env_var);
                    let mut input = String::new();
                    print!("Enter value for `{}`: ", env_var);
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim().to_string();

                    print!("You entered: {}. Is this correct? (y/n): ", input);
                    io::stdout().flush()?;
                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm)?;
                    if confirm.trim().to_lowercase() == "y" {
                        println!("Environment variable {} set to: {}", env_var, input);
                        std::env::set_var(env_var, input);
                    } else {
                        println!("Skipping setting environment variable {}.", env_var);
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupEntry {
    check: Option<String>,
    commands: Vec<CommandStruct>,
    configs: Option<Vec<ConfigItem>>,
    setup: Option<SetupItem>,
    description: String,
}
impl SetupEntry {
    pub fn get_description(&self) -> &String {
        &self.description
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

    pub fn remove_command(&mut self, index: usize) {
        self.commands.remove(index);
    }

    pub fn clear_commands(&mut self) {
        let mut commands_to_remove = Vec::new();
        for (index, command) in self.commands.iter().enumerate() {
            if command.should_skip() {
                commands_to_remove.push(index);
            }
        }

        for index in commands_to_remove.iter().rev() {
            self.remove_command(*index);
        }
    }
}

impl CommandRunner for SetupEntry {
    fn run(&self) -> Status {
        let mut process = Status::Running;

        if let Some(check) = &self.check {
            if let Ok(result) = CommandStruct::validate_command(&check, |output| {
                !String::from_utf8_lossy(&output.stdout).is_empty()
            }) {
                if result {
                    process = Status::Success;
                }
            }
        }

        if process != Status::Success {
            process = self.run_commands();
        }

        if self.configs.is_some() && process != Status::Failure {
            process = self.run_configs();
        }

        process
    }
}

impl ExecutableSetup for SetupEntry {
    fn setup(&mut self) -> Status {
        self.clear_commands();

        Status::Running.print_message(&format!("Setup: {:?}", self.description));
        if let Some(setup) = &mut self.setup {
            if let Err(e) = setup.ensure_working_dir() {
                eprintln!("Error creating working directory: {}", e);
                return Status::Failure;
            }

            if let Err(e) = setup.ensure_env_vars() {
                eprintln!("Error setting environment variables: {}", e);
                return Status::Failure;
            }
        }

        self.run()
    }
}
