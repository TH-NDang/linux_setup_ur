use std::{cell::RefCell, error, io, process};

use serde::{Deserialize, Serialize};

use super::shell::Shell;
use crate::{
    distribution::identify_linux_distribution, traits::ProcessRunner, utils::Status, CommandRunner,
    DistributionType, ErrorHandler,
};

const COMMAND_NOT_FOUND: &str = "Command not found";
const COMMAND_EXECUTION_FAILED: &str = "Command execution failed";

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandStruct {
    command: String,
    shell: Option<Shell>,
    distribution: Option<DistributionType>,
    #[serde(skip)]
    status: RefCell<Status>,
    check: Option<String>,
    #[serde(skip)]
    run_spawn: Option<bool>,
}
impl CommandStruct {
    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn should_skip(&self) -> bool {
        if let Some(distribution) = &self.distribution {
            if *distribution != identify_linux_distribution() {
                return true;
            }
        }
        false
    }

    fn set_status(&self, status: Status, message: &str) {
        self.status.replace(status);
        self.status.borrow().print_message(message);
    }

    fn validate_command(
        &self,
        check: impl Fn(process::Output) -> bool,
    ) -> Result<bool, Box<dyn error::Error>> {
        let output = process::Command::new("sh")
            .arg("-c")
            .arg(&self.check.as_ref().unwrap())
            .output()?;

        Ok(output.status.success() && check(output))
    }

    pub fn distribution(&self) -> Option<&DistributionType> {
        self.distribution.as_ref()
    }
}

impl CommandRunner for CommandStruct {
    fn setup_command(&self) -> process::Command {
        let mut command =
            process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string());
        command.arg("-c").arg(&self.command);
        command
    }

    fn is_run_spawn(&self) -> bool {
        self.run_spawn.unwrap_or(false)
    }
}

impl ProcessRunner for CommandStruct {
    fn before_run(&self) -> Status {
        if self.should_skip() {
            return Status::Skipped;
        }

        if self.check.is_some() {
            if let Ok(result) =
                self.validate_command(|output| !String::from_utf8_lossy(&output.stdout).is_empty())
            {
                if result {
                    self.set_status(Status::Passed, &format!("{}", self.command));
                    return Status::Passed;
                }
            }
        }

        Status::Success
    }

    fn after_run(&self, command_status: Status) -> Status {
        self.status.replace(command_status.clone());

        match command_status {
            Status::Failure => Status::Failure,
            Status::Skipped => Status::Skipped,
            Status::Passed => Status::Passed,
            _ => Status::Success,
        }
    }
}

impl ErrorHandler for CommandStruct {
    fn handle_command_error(stderr: &str) -> io::Error {
        if stderr.contains(COMMAND_NOT_FOUND) {
            io::Error::new(io::ErrorKind::NotFound, COMMAND_NOT_FOUND)
        } else {
            io::Error::new(io::ErrorKind::Other, COMMAND_EXECUTION_FAILED)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::process::Output;

    #[test]
    fn test_validate_command_success() {
        let command = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
            check: Some("echo true".to_string()),
            run_spawn: Some(false),
        };

        let check =
            |output: Output| -> bool { String::from_utf8_lossy(&output.stdout).contains("true") };

        let result = command.validate_command(check);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_validate_command_failure() {
        let command = CommandStruct {
            command: "invalid_command".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
            check: Some("echo".to_string()),
            run_spawn: Some(false),
        };

        let check =
            |output: Output| -> bool { String::from_utf8_lossy(&output.stdout).contains("Hello") };

        let result = command.validate_command(check);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_run_success() {
        let command_struct = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
            check: None,
            run_spawn: Some(false),
        };

        let status = command_struct.run();
        assert_eq!(status, Status::Success);
    }

    #[test]
    fn test_run_failure() {
        let command_struct = CommandStruct {
            command: "invalid_command".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
            check: None,
            run_spawn: Some(false),
        };

        let status = command_struct.run();
        assert_eq!(status, Status::Failure);
    }

    #[test]
    fn test_run_use_zsh() {
        use std::fs;
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let zshrc_path = Path::new(".zshrc");
        let mut file = File::create(&zshrc_path).expect("Unable to create .zshrc file");
        writeln!(file, "echo 'Hello from .zshrc'").expect("Unable to write to .zshrc file");

        let command_struct = CommandStruct {
            command: format!("source {}", zshrc_path.display()),
            shell: Some(Shell::Zsh),
            distribution: None,
            status: RefCell::new(Status::Normal),
            check: None,
            run_spawn: Some(false),
        };

        let status = command_struct.run();
        assert_eq!(status, Status::Success);

        fs::remove_file(zshrc_path).expect("Unable to delete .zshrc file");
    }
}
