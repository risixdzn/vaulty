use std::{ fs, io };
use colored::Colorize;
use md5::compute;
use tabled::{ settings::Style, Table };
use inquire::Text;

use crate::{ types::{ Command, CommandList }, utils::padded_println };

/* Generates a 6 length hash to reference the commands */
fn generate_hash(command: &str) -> String {
    let hash = format!("{:x}", compute(command));
    return hash[..6].to_string();
}

/* 
    Adds a new command to the command list
    If the list is not created, it will be initalized at the user AppData folder.
*/
pub fn save(command: &str, description: &str) -> io::Result<()> {
    /* Get the platform-specific app data directory */
    let proj_dirs = directories::ProjectDirs
        ::from("com", "vaulty", "vaulty")
        .expect("Failed to get project directories.");

    let data_dir = proj_dirs.data_dir();
    let file_path = data_dir.join("commands.json");

    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }

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
        id: hash.clone(),
        command: command.to_string(),
        description: description.to_string(),
    };

    command_list.commands.push(new_command.clone());

    let json_data = serde_json::to_string_pretty(&command_list)?;
    fs::write(&file_path, json_data)?;

    /* Creates and displays a table with the new command */
    let mut table = Table::new(vec![new_command]);
    table.with(Style::rounded());

    padded_println(
        vec![
            "✔ Command added!".green().to_string(),
            table.to_string(),
            "\n💡 Use 'vaulty list' to see your stored commands.".yellow().to_string()
        ]
    );

    return Ok(());
}

pub fn interactive_save() -> io::Result<()> {
    padded_println(vec!["Vaulty - Adding command".cyan().to_string()]);

    let command = Text::new("Enter the command to be saved:")
        .prompt()
        .expect("Failed to prompt for command");
    let description = Text::new("Enter a description for the command:")
        .prompt()
        .expect("Failed to prompt for description");

    save(&command, &description)
}
