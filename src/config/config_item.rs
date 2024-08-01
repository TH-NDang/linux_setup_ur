use serde::{Deserialize, Serialize};

use crate::{utils::Status, CommandStruct, Configurator};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigItem {
    check: Option<String>,
    command: CommandStruct,
}

impl Configurator for ConfigItem {
    fn apply(&self) -> Status {
        if let Some(check) = &self.check {
            match CommandStruct::validate_command(check, |output| {
                !String::from_utf8_lossy(&output.stdout).is_empty()
            }) {
                Ok(result) => {
                    if result {
                        return Status::Success;
                    }
                }
                Err(e) => {
                    Status::Failure.print_message(&format!("Error validating check: {}", e));
                    return Status::Failure;
                }
            }
        }

        Status::Running.print_message(&format!("==> Applying config: {}", self.command.command()));
        self.command.interact_mode()
    }

    fn revert(&self) -> Status {
        todo!()
    }
}
