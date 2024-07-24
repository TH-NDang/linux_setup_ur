use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

/// Implements the `Display` trait for the Color enum, allowing custom formatting of Color values.
enum Color {
    Yellow,
    Green,
    Red,
    Blue,
    None,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f, "\x1b[33m"),
            Color::Green => write!(f, "\x1b[32m"),
            Color::Red => write!(f, "\x1b[31m"),
            Color::Blue => write!(f, "\x1b[34m"),
            Color::None => write!(f, "\x1b[0m"),
        }
    }
}

/// Defines an enum representing different statuses of a command execution.
/// Implements `print_message(message: &str)` methods to print messages based on the command status.
pub enum CommandStatus {
    Running,
    Success,
    Warning,
    Failure,
    Normal,
}

impl CommandStatus {
    pub fn print_message(&self, message: &str) {
        match self {
            CommandStatus::Running => println!(
                "{}==> ⏳Running: {}{}",
                CommandStatus::Running,
                message,
                CommandStatus::Normal
            ),
            CommandStatus::Success => println!(
                "{}==> ✅Succeeded: {}{}",
                CommandStatus::Success,
                message,
                CommandStatus::Normal
            ),
            CommandStatus::Warning => println!(
                "{}==> ⚠️Warning: {}{}",
                CommandStatus::Warning,
                message,
                CommandStatus::Normal
            ),
            CommandStatus::Failure => eprintln!(
                "{}==> ❌Failed: {}{}",
                CommandStatus::Failure,
                message,
                CommandStatus::Normal
            ),
            CommandStatus::Normal => println!("{}", message),
        }
    }
}

impl Display for CommandStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandStatus::Running => write!(f, "{}", Color::Blue),
            CommandStatus::Success => write!(f, "{}", Color::Green),
            CommandStatus::Warning => write!(f, "{}", Color::Yellow),
            CommandStatus::Failure => write!(f, "{}", Color::Red),
            CommandStatus::Normal => write!(f, "{}", Color::None),
        }
    }
}

pub enum DistributionType {
    Ubuntu,
    ArchLinux,
    Unknown,
}

pub trait LinuxDistributor {
    fn check() -> Self;
}
impl LinuxDistributor for DistributionType {
    fn check() -> Self {
        if file_exists("/etc/arch-release") {
            DistributionType::ArchLinux
        } else if file_exists("/etc/lsb-release") {
            match read_file_content("/etc/lsb-release") {
                Ok(content) => {
                    if content.contains("DISTRIB_ID=Ubuntu") {
                        DistributionType::Ubuntu
                    } else {
                        DistributionType::Unknown
                    }
                }
                Err(_) => DistributionType::Unknown,
            }
        } else {
            DistributionType::Unknown
        }
    }
}
impl Display for DistributionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributionType::Ubuntu => write!(f, "Ubuntu"),
            DistributionType::ArchLinux => write!(f, "Arch Linux"),
            DistributionType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Checks if a file exists at the specified path.
///
/// ### Arguments
///
/// * `path` - A string slice that holds the path to the file.
///
/// ### Returns
///
/// A boolean value indicating whether the file exists or not.
fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Reads the content of a file specified by the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// A `Result` containing a `String` with the content of the file if successful, or an `std::io::Error` if an error occurs.
fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Identifies the Linux distribution by calling the `check` method of `DistributionType`.
pub fn identify_linux_distribution() -> DistributionType {
    DistributionType::check()
}

pub trait CommandRunner {
    fn run(&self);
}

pub struct CommandStruct {
    command: String,
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
        let output = process::Command::new("sh")
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

    pub fn interact_mode(&self) {
        let mut output = process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .spawn()
            .expect("Failed to execute command");

        let status = output.wait().expect("Failed to wait on child");

        if status.success() {
            CommandStatus::Success.print_message(&self.command);
        } else {
            CommandStatus::Failure.print_message(&self.command);
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn set_command(&mut self, command: &str) {
        self.command = command.to_string();
    }
}
impl CommandRunner for CommandStruct {
    fn run(&self) {
        CommandStatus::Running.print_message(&self.command);
        match self.execute_command() {
            Ok(_) => {
                CommandStatus::Success.print_message(&self.command);
            }
            Err(_) => {
                CommandStatus::Failure.print_message(&self.command);
            }
        }
    }
}

pub struct CommandFactory;
impl CommandFactory {
    pub fn new(command: &str) -> CommandStruct {
        CommandStruct {
            command: command.to_string(),
        }
    }
}

pub struct CommandRepository {
    commands: Vec<CommandStruct>,
}

impl From<Vec<&str>> for CommandRepository {
    fn from(value: Vec<&str>) -> Self {
        let mut repo = CommandRepository::new();
        for command in value {
            repo.add_command(CommandFactory::new(command));
        }
        repo
    }
}

impl CommandRepository {
    pub fn new() -> Self {
        CommandRepository {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: CommandStruct) {
        self.commands.push(command);
    }

    pub fn run_commands(&self) {
        for command in &self.commands {
            command.run();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_linux_distribution() {
        assert_eq!(format!("{}", DistributionType::Ubuntu), "Ubuntu");
        assert_eq!(format!("{}", DistributionType::ArchLinux), "Arch Linux");
        assert_eq!(format!("{}", DistributionType::Unknown), "Unknown");
    }

    #[test]
    fn test_file_exists() {
        assert!(file_exists("/etc/passwd"));
        assert!(!file_exists("/nonexistent-file"));
    }

    #[test]
    fn test_read_file_content() {
        let content = read_file_content("/etc/passwd");
        assert!(content.is_ok());
        let content = read_file_content("/nonexistent-file");
        assert!(content.is_err());
    }

    #[test]
    fn test_execute_command() {
        let command_success = CommandStruct {
            command: "echo Hello, world!".to_string(),
        };
        let result = command_success.execute_command();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");

        let command_failure = CommandStruct {
            command: "nonexistentcommand".to_string(),
        };
        let result = command_failure.execute_command();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_identify_linux_distribution() {
        // This test is environment-dependent and may need to be adjusted based on the actual system
        let distro = identify_linux_distribution();
        assert!(matches!(
            distro,
            DistributionType::Ubuntu | DistributionType::ArchLinux | DistributionType::Unknown
        ));
    }

    #[test]
    fn test_run_command() {
        let command = CommandStruct {
            command: "echo Hello, world!".to_string(),
        };
        command.run();
    }

    #[test]
    fn test_command_struct_command() {
        let command = CommandStruct {
            command: "ls".to_string(),
        };
        assert_eq!(command.command(), "ls");
    }

    #[test]
    fn test_command_factory_new() {
        let command = CommandFactory::new("ls");
        assert_eq!(command.command(), "ls");
    }

    #[test]
    fn test_command_repository_add_and_run_commands() {
        let mut repo = CommandRepository::new();
        let command1 = CommandFactory::new("echo Command 1");
        let command2 = CommandFactory::new("echo Command 2");

        repo.add_command(command1);
        repo.add_command(command2);

        assert_eq!(repo.commands.len(), 2);

        repo.run_commands();
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
        repo.add_command(command);
        assert_eq!(repo.commands.len(), 1);
    }

    #[test]
    fn test_display_color() {
        assert_eq!(format!("{}", Color::Yellow), "\x1b[33m");
        assert_eq!(format!("{}", Color::Green), "\x1b[32m");
        assert_eq!(format!("{}", Color::Red), "\x1b[31m");
        assert_eq!(format!("{}", Color::Blue), "\x1b[34m");
        assert_eq!(format!("{}", Color::None), "\x1b[0m");
    }

    #[test]
    fn test_display_command_status() {
        assert_eq!(format!("{}", CommandStatus::Running), "\x1b[34m");
        assert_eq!(format!("{}", CommandStatus::Success), "\x1b[32m");
        assert_eq!(format!("{}", CommandStatus::Warning), "\x1b[33m");
        assert_eq!(format!("{}", CommandStatus::Failure), "\x1b[31m");
        assert_eq!(format!("{}", CommandStatus::Normal), "\x1b[0m");
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
