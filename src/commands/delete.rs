use std::{ fs, io };
use colored::*;
use inquire::{ Confirm, Select };

use crate::{ types::CommandList, utils::padded_println };

use super::get_commands;

/* 
    Delete a specified command from the CommandList
*/
pub fn delete(id: &str, confirm_deletion: &bool) -> io::Result<()> {
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
        return Ok(());
    }

    let mut command_list: CommandList = {
        let file_content = fs::read_to_string(&file_path)?;
        serde_json::from_str(&file_content)?
    };

    /* Filter the command list to find the command by the given id */
    if let Some(command) = command_list.commands.iter().find(|c| c.id == id) {
        match confirm_deletion {
            true => {
                command_list.commands.retain(|c| c.id != id);

                let updated_file_content = serde_json::to_string(&command_list)?;
                fs::write(&file_path, updated_file_content)?;

                padded_println(vec!["✔ Command deleted successfully.".green().to_string()]);
                return Ok(());
            }
            false => (),
        }

        let ans = Confirm::new("Delete the following command?")
            .with_default(false)
            .with_help_message(
                &format!(
                    "This command will be deleted: {} ({})",
                    command.command,
                    command.description
                )
            )
            .prompt();

        match ans {
            Ok(true) => {
                command_list.commands.retain(|c| c.id != id);

                let updated_file_content = serde_json::to_string(&command_list)?;
                fs::write(&file_path, updated_file_content)?;

                padded_println(vec!["✔ Command deleted successfully.".green().to_string()]);
                return Ok(());
            }
            Ok(false) => {
                padded_println(vec!["Exited without deleting the command.".yellow().to_string()]);
                return Ok(());
            }
            Err(_) => {
                padded_println(vec!["Error with questionnaire, try again later".red().to_string()]);
                return Ok(());
            }
        }
    } else {
        padded_println(vec!["⚠️ Command not found".yellow().to_string()]);
    }

    return Ok(());
}

pub fn interactive_delete() -> io::Result<()> {
    let command_list = get_commands()?;

    if command_list.commands.is_empty() {
        padded_println(
            vec![
                "⚠️ No commands found".yellow().to_string(),
                "\n💡 Use 'vaulty save' to add a new command.".yellow().to_string()
            ]
        );
        return Ok(());
    }

    let options: Vec<String> = command_list.commands
        .iter()
        .map(|c| format!("{} - {}", c.command, c.description.italic().bright_blue()))
        .collect();

    let selection = Select::new(
        &"👉 Select a command to delete:".yellow().to_string(),
        options
    ).prompt();

    match selection {
        Ok(selected) => {
            /*  Find the command's ID based on the selected entry */
            if
                let Some(command) = command_list.commands
                    .iter()
                    .find(|c| { selected.starts_with(&c.command) }) // Match the start of the formatted `${cmd} - ${desc} with the pure command`
            {
                delete(&command.id, &false)?;
            }
        }
        Err(_) => {
            padded_println(vec!["❌ Error occurred while selecting.".red().to_string()]);
        }
    }

    Ok(())
}
