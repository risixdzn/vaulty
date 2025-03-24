use serde::{ Deserialize, Serialize };
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Command {
    pub id: String,
    pub command: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList {
    pub commands: Vec<Command>,
}
