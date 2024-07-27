use serde::{Deserialize, Serialize};

use crate::{utils::Status, CommandStruct, Configurator};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigItem {
    pub check: Option<String>,
    pub command: CommandStruct,
}

impl Configurator for ConfigItem {
    fn apply(&self) -> Status {
        if let Some(check) = &self.check {
            if CommandStruct::check_command_success(check, |output| {
                !String::from_utf8_lossy(&output.stdout).is_empty()
            }) {
                return Status::Success;
            }
        }

        self.command.interact_mode();
        Status::Normal
    }

    fn revert(&self) -> Status {
        todo!()
    }
}
