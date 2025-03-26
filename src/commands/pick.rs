use std::io;

use arboard::Clipboard;
use colored::Colorize;
use inquire::Select;

use crate::utils::padded_println;

use super::get_commands;

/*
    Pick a command from the CommandList, will copy to clipboard.
*/
pub fn pick() -> io::Result<()> {
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

    let selection = Select::new(&"👉 Pick a command:".yellow().to_string(), options).prompt();

    match selection {
        Ok(selected) => {
            /*  Find the command's ID based on the selected entry */
            if
                let Some(command) = command_list.commands
                    .iter()
                    .find(|c| { selected.starts_with(&c.command) }) // Match the start of the formatted `${cmd} - ${desc} with the pure command`
            {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(command.command.to_owned()).unwrap();
                padded_println(
                    vec![
                        "✔ Command copied to clipboard 📋".green().to_string(),
                        "💡 Press CTRL+V (or right click) to paste it.".yellow().to_string()
                    ]
                );
            }
        }
        Err(_) => {
            padded_println(vec!["❌ Error occurred while selecting.".red().to_string()]);
        }
    }

    Ok(())
}
