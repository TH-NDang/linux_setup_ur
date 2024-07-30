use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

use crate::setup::SetupEntry;
use crate::traits::executable_setup::ExecutableSetup;
use crate::Repository;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupRegistry {
    entries: Vec<SetupEntry>,
}

impl SetupRegistry {
    pub fn load_from_json(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open file");
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to parse JSON")
    }

    pub fn execute(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.setup();
        }
    }
}

impl Repository<SetupEntry> for SetupRegistry {
    fn new() -> Self {
        SetupRegistry {
            entries: Vec::new(),
        }
    }

    fn add(&mut self, item: SetupEntry) {
        self.entries.push(item);
    }
}
