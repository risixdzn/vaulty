use std::{ fs, io };
use colored::*;

use crate::{ types::CommandList, utils::{ self, padded_println } };

pub fn get_commands() -> io::Result<CommandList> {
    /* Get the platform-specific app data directory */
    let proj_dirs = directories::ProjectDirs
        ::from("com", "vaulty", "vaulty")
        .expect("Failed to get project directories.");

    let data_dir = proj_dirs.data_dir();
    let file_path = data_dir.join("commands.json");

    if !data_dir.exists() || !file_path.exists() {
        padded_println(
            vec![
                "⚠️ No commands found".yellow().to_string(),
                "\n💡 Use 'vaulty save' to add a new command.".yellow().to_string()
            ]
        );
        return Ok(CommandList { commands: Vec::new() });
    }

    let command_list: CommandList = {
        let file_content = fs::read_to_string(&file_path)?;
        serde_json::from_str(&file_content)?
    };

    return Ok(command_list);
}

/* 
    Lists all the commands stored in the command list
*/
pub fn list() -> io::Result<()> {
    let command_list = get_commands()?;

    if command_list.commands.len() == 0 {
        padded_println(
            vec![
                "⚠️ No commands found".yellow().to_string(),
                "\n💡 Use 'vaulty save' to add a new command.".yellow().to_string()
            ]
        );
        return Ok(());
    }

    /* Creates and displays a table with all the commands */

    let table = utils::render_table(command_list.commands);

    padded_println(
        vec![
            "✔ Your saved commands:\n".green().to_string(),
            table,
            "\n💡 Use 'vaulty pick <id>' to run a specific command.".yellow().to_string()
        ]
    );

    return Ok(());
}
