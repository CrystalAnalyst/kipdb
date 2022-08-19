use serde::{Deserialize, Serialize};
use clap::{Subcommand};

#[derive(Serialize, Deserialize, Debug, Subcommand)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
    Get { key: String }
}

impl Command {
    pub fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    pub fn remove(key: String) -> Command {
        Command::Remove { key }
    }

    pub fn get(key: String) -> Command {
        Command::Get { key }
    }
}

