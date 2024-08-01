use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Shell {
    Bash,
    Zsh,
    #[default]
    Sh,
    Custom(String),
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::Bash => write!(f, "bash"),
            Shell::Zsh => write!(f, "zsh"),
            Shell::Sh => write!(f, "sh"),
            Shell::Custom(shell) => write!(f, "{}", shell),
        }
    }
}
