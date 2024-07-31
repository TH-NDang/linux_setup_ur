use std::{error, io, process};

use serde::{Deserialize, Serialize};

use super::shell::Shell;
use crate::{
    distribution::identify_linux_distribution, utils::Status, CommandRunner, DistributionType,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandStruct {
    command: String,
    shell: Option<Shell>,
    distribution: Option<DistributionType>,
}
impl CommandStruct {
    fn execute_command(&self) -> Result<String, io::Error> {
        if let Some(distribution) = &self.distribution {
            if *distribution != identify_linux_distribution() {
                // Status::Skipped.print_message(&self.command);
                // return Status::Skipped;
                return Ok("Skipped".to_string());
            }
        }

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
        if let Some(distribution) = &self.distribution {
            if *distribution != identify_linux_distribution() {
                // Status::Skipped.print_message(&self.command);
                // return Status::Skipped;
                return Status::Normal;
            }
        }

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
    use std::process::Output;
    use std::io;

    #[test]
    fn test_execute_command_success() {
        let command_struct = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
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
        };

        let result = command_struct.execute_command();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_interact_mode_success() {
        let command_struct = CommandStruct {
            command: "echo Hello".to_string(),
            shell: Some(Shell::Sh),
            distribution: None,
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
        };

        let status = command_struct.interact_mode();
        assert_eq!(status, Status::Failure);
    }

    #[test]
    fn test_validate_command_success() {
        let command = "echo Hello";
        let check = |output: Output| -> bool { String::from_utf8_lossy(&output.stdout).contains("Hello") };

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
        };

        let status = command_struct.run();
        assert_eq!(status, Status::Failure);
    }

    #[test]
    fn test_run_use_zsh() {
        let command_struct = CommandStruct {
            command: "source ~/.zshrc".to_string(),
            shell: Some(Shell::Zsh),
            distribution: None,
        };

        let status = command_struct.execute_command();
        assert!(status.is_ok());
    }
}
