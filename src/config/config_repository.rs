use serde::{Deserialize, Serialize};

use crate::utils::Status;

use crate::Configurator;
use crate::{CommandRunner, ConfigItem, Repository};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigRepository {
    configs: Vec<ConfigItem>,
}

impl Repository<ConfigItem> for ConfigRepository {
    fn new() -> Self {
        ConfigRepository {
            configs: Vec::new(),
        }
    }

    fn add(&mut self, item: ConfigItem) {
        self.configs.push(item);
    }
}

impl CommandRunner for ConfigRepository {
    fn run(&self) -> Status {
        let failed = self
            .configs
            .iter()
            .filter(|config| config.apply() == Status::Failure)
            .count();

        if failed > 0 {
            Status::Failure
        } else {
            Status::Success
        }
    }
}
