use crate::CommandStruct;

pub struct CommandFactory;
impl CommandFactory {
    pub fn new(command: &str) -> CommandStruct {
        CommandStruct {
            command: command.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_factory_new() {
        let command = CommandFactory::new("ls");
        assert_eq!(command.command(), "ls");
    }
}
