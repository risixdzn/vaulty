use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub hash: String,
    pub command: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList {
    pub commands: Vec<Command>,
}