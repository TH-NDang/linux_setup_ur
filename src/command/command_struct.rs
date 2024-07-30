use std::{error, io, process};

use serde::{Deserialize, Serialize};

use super::shell::Shell;
use crate::{utils::Status, CommandRunner};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandStruct {
    pub command: String,
    pub shell: Option<Shell>,
}
impl CommandStruct {
    /// Executes the command stored in the `CommandStruct` instance and returns the output as a `Result`
    ///
    /// ### Success
    ///
    /// Returns a `String` containing the output of the command if the command execution is successful.
    ///
    /// ### Errors
    ///
    /// Returns an `io::Error` if the command execution fails or if the command is not found.
    ///
    fn execute_command(&self) -> Result<String, io::Error> {
        let output = process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string())
            .arg("-c")
            .arg(&self.command)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("command not found") {
                Err(io::Error::new(io::ErrorKind::NotFound, "Command not found"))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Command execution failed",
                ))
            }
        }
    }

    pub fn interact_mode(&self) -> Status {
        let mut output =
            process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string())
                .arg("-c")
                .arg(&self.command)
                .spawn()
                .expect("Failed to execute command");

        let status = match output.wait() {
            Ok(status) => status,
            Err(err) => {
                eprintln!("Failed to wait on child: {}", err);
                Status::Failure.print_message(&self.command);
                return Status::Failure;
            }
        };

        match status.code() {
            Some(0) => {
                Status::Success.print_message(&self.command);
                Status::Success
            }
            Some(_) => {
                Status::Failure.print_message(&self.command);
                Status::Failure
            }
            None => {
                eprintln!("Command terminated by signal");
                Status::Failure.print_message(&self.command);
                Status::Failure
            }
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn set_command(&mut self, command: &str) {
        self.command = command.to_string();
    }

    pub fn validate_command(
        command: &str,
        check: impl Fn(process::Output) -> bool,
    ) -> Result<bool, Box<dyn error::Error>> {
        Status::Running.print_message(&format!("==> Checking command: {}", command));

        let output = match process::Command::new("sh").arg("-c").arg(command).output() {
            Ok(output) => output,
            Err(e) => {
                Status::Failure.print_message(&format!("Command failed: {}", command));
                return Err(Box::new(e));
            }
        };

        if output.status.success() && check(output) {
            Status::Success.print_message(&format!("\t--> Checked is true: {}", command));
            Ok(true)
        } else {
            Status::Failure.print_message(&format!("\t--> Checked is false: {}", command));
            Ok(false)
        }
    }
}
impl CommandRunner for CommandStruct {
    fn run(&self) -> Status {
        Status::Running.print_message(&self.command);
        match self.execute_command() {
            Ok(_) => {
                Status::Success.print_message(&self.command);
                Status::Success
            }
            Err(_) => {
                Status::Failure.print_message(&self.command);
                Status::Failure
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command() {
        let command_success = CommandStruct {
            command: "echo Hello, world!".to_string(),
            shell: None,
        };
        let result = command_success.execute_command();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");

        let command_failure = CommandStruct {
            command: "nonexistentcommand".to_string(),
            shell: None,
        };
        let result = command_failure.execute_command();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_run_command() {
        let command = CommandStruct {
            command: "echo Hello, world!".to_string(),
            shell: None,
        };
        command.run();
    }

    #[test]
    fn test_command_struct_command() {
        let command = CommandStruct {
            command: "ls".to_string(),
            shell: None,
        };
        assert_eq!(command.command(), "ls");
    }
}
