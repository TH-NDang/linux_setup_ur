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

    fn handle_command_error(&self, output: process::Output) -> Result<String, io::Error> {
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

    fn spawn_command(&self) -> Result<process::Child, io::Error> {
        process::Command::new(self.shell.as_ref().unwrap_or(&Shell::Sh).to_string())
            .arg("-c")
            .arg(&self.command)
            .spawn()
    }

    fn handle_status(&self, status: process::ExitStatus) -> Status {
        match status.code() {
            Some(0) => Status::Success,
            Some(_) => Status::Failure,
            None => {
                eprintln!("Command terminated by signal");
                Status::Failure
            }
        }
    }

    fn execute_command(&self) -> Result<String, io::Error> {
        if self.should_skip() {
            return Ok("Skipped".to_string());
        }

        let output = self.run_command()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            self.handle_command_error(output)
        }
    }

    pub fn interact_mode(&self) -> Status {
        if self.should_skip() {
            return Status::Normal;
        }

        let mut output = match self.spawn_command() {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
                return Status::Failure;
            }
        };

        match output.wait() {
            Ok(status) => self.handle_status(status),
            Err(err) => {
                eprintln!("Failed to wait on child: {}", err);
                Status::Failure
            }
        }
    }

    pub fn validate_command(
        command: &str,
        check: impl Fn(process::Output) -> bool,
    ) -> Result<bool, Box<dyn error::Error>> {
        Status::Running.print_message(&format!("==> Checking command: {}", command));

        let output = process::Command::new("sh").arg("-c").arg(command).output()?;

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
        };

        let status = command_struct.execute_command();
        assert!(status.is_ok());

        fs::remove_file(zshrc_path).expect("Unable to delete .zshrc file");
    }
}
