use std::{cell::RefCell, error, io, process};

use serde::{Deserialize, Serialize};

use super::shell::Shell;
use crate::{
    distribution::identify_linux_distribution, utils::Status, CommandRunner, DistributionType,
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
}
impl CommandStruct {
    pub fn command(&self) -> &str {
        &self.command
    }

    fn should_skip(&self) -> bool {
        if let Some(distribution) = &self.distribution {
            if *distribution != identify_linux_distribution() {
                return true;
            }
        }
        false
    }

    fn run_command(&self) -> Result<process::Output, io::Error> {
        process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string())
            .arg("-c")
            .arg(&self.command)
            .output()
    }

    fn handle_command_error(&self, stderr: &str) -> io::Error {
        if stderr.contains(COMMAND_NOT_FOUND) {
            io::Error::new(io::ErrorKind::NotFound, COMMAND_NOT_FOUND)
        } else {
            io::Error::new(io::ErrorKind::Other, COMMAND_EXECUTION_FAILED)
        }
    }

    fn spawn_command(&self) -> Result<process::Child, io::Error> {
        process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string())
            .arg("-c")
            .arg(&self.command)
            .spawn()
    }

    fn handle_status(&self, exit_status: &std::process::ExitStatus) -> Status {
        match exit_status.code() {
            Some(0) => {
                self.set_status(Status::Success, &format!("{}", self.command));
                self.status.borrow().clone()
            }
            Some(_) => {
                self.set_status(
                    Status::Failure,
                    &format!("{} [Exit code: {:?}]", self.command, exit_status.code()),
                );
                self.status.borrow().clone()
            }
            None => {
                self.set_status(
                    Status::Failure,
                    &format!("{} [Command terminated by signal]", self.command),
                );
                self.status.borrow().clone()
            }
        }
    }

    fn set_status(&self, status: Status, message: &str) {
        self.status.replace(status);
        self.status.borrow().print_message(message);
    }

    fn execute_command(&self) -> io::Result<String> {
        if self.should_skip() {
            self.status.replace(Status::Skipped);
            return Ok("Skipped".to_string());
        }

        let output = self.run_command()?;
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            self.set_status(Status::Success, &format!("{}", self.command));
            Ok(output_str)
        } else {
            self.set_status(Status::Failure, &format!("{}", self.command));
            Err(self.handle_command_error(&String::from_utf8_lossy(&output.stderr)))
        }
    }

    pub fn interact_mode(&self) -> Status {
        if self.should_skip() {
            self.status.replace(Status::Skipped);
            return self.status.borrow().clone();
        }

        let mut child = match self.spawn_command() {
            Ok(child) => child,
            Err(err) => {
                self.set_status(Status::Failure, &format!("Error: {}", err));
                return self.status.borrow().clone();
            }
        };

        match &child.wait() {
            Ok(status) => self.handle_status(status),
            Err(err) => {
                self.set_status(Status::Failure, &format!("Error to wait on child: {}", err));
                self.status.borrow().clone()
            }
        }
    }

    pub fn validate_command(
        command: &str,
        check: impl Fn(process::Output) -> bool,
    ) -> Result<bool, Box<dyn error::Error>> {
        let output = process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        Ok(output.status.success() && check(output))
    }
}
impl CommandRunner for CommandStruct {
    fn run(&self) -> Status {
        match self.execute_command() {
            Ok(_) => self.status.borrow().clone(),
            Err(e) => {
                self.set_status(Status::Failure, &format!("{}", self.command));
                eprintln!("Error: {}", e);
                Status::Failure
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::process::Output;

    #[test]
    fn test_execute_command_success() {
        let command_struct = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
        };

        let result = command_struct.execute_command();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_execute_command_failure() {
        let command_struct = CommandStruct {
            command: "invalid_command".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
        };

        let result = command_struct.execute_command();
        assert!(result.is_err());
    }

    #[test]
    fn test_interact_mode_success() {
        let command_struct = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
        };

        let status = command_struct.interact_mode();
        assert_eq!(status, Status::Success);
    }

    #[test]
    fn test_interact_mode_failure() {
        let command_struct = CommandStruct {
            command: "invalid_command".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
            status: RefCell::new(Status::Normal),
        };

        let status = command_struct.interact_mode();
        assert_eq!(status, Status::Failure);
    }

    #[test]
    fn test_validate_command_success() {
        let command = "echo Hello";
        let check =
            |output: Output| -> bool { String::from_utf8_lossy(&output.stdout).contains("Hello") };

        let result = CommandStruct::validate_command(command, check);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_validate_command_failure() {
        let command = "invalid_command";
        let check = |_output: Output| -> bool { false };

        let result = CommandStruct::validate_command(command, check);
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
        };

        let status = command_struct.run();
        assert_eq!(status, Status::Success);

        fs::remove_file(zshrc_path).expect("Unable to delete .zshrc file");
    }
}
