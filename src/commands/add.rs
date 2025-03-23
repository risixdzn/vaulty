use std::{fs, io};
use colored::Colorize;
use md5::compute;
use tabled::{settings::Style, Table};

use crate::{types::{Command, CommandList}, utils::padding};

/* Generates a 6 length hash to reference the commands */
fn generate_hash(command: &str) -> String {
    let hash = format!("{:x}", compute(command));
    return hash[..6].to_string()
}

/* 
    Adds a new command to the command list
    If the list is not created, it will be initalized at the user AppData folder.
*/
pub fn add(command: &str, description: &str) -> io::Result<()> {
    /* Get the platform-specific app data directory */
    let proj_dirs = directories::ProjectDirs::from(
            "com",
            "vaulty",
            "vaulty")
        .expect("Failed to get project directories.");

    let data_dir = proj_dirs.data_dir();
    let file_path = data_dir.join("commands.json");


    let hash = generate_hash(command);

    /* Check if the file exists, grabbing the content or initializing a new empty CommandList */
    let mut command_list: CommandList = if file_path.exists() {
        let file_content = fs::read_to_string(&file_path)?;
        serde_json::from_str(&file_content)?
    } else {
        CommandList { commands: Vec::new() }
    };

    /* Creates and pushes the new object to the command list, writing to the file */
    let new_command = Command {
        hash: hash.clone(),
        command: command.to_string(),
        description: description.to_string(),
    };

    command_list.commands.push(new_command.clone());

    let json_data = serde_json::to_string_pretty(&command_list)?;
    fs::write(&file_path, json_data)?;

    /* Creates and displays a table with the new command */
    let mut table = Table::new(vec![new_command]);
    table.with(Style::rounded());

    padding(vec![
        "✔ Command added!".green().to_string(),
        table.to_string(),
        "\n👉 Use 'vaulty list' to see your stored commands.".yellow().to_string(),
    ]);

    return Ok(())
}

