use directories::ProjectDirs;
use std::fs;
use serde_json;
use crate::{types::CommandList, utils::padding};
use colored::*;

pub fn init() -> std::io::Result<()> {

    /*
        Get the platform-specific app data directory
        Create the app data directory if it doesn't exist
        Write the initial commands.json file
    */
    if let Some(proj_dirs) = ProjectDirs::from("com", "vaulty", "vaulty") {
        let data_dir = proj_dirs.data_dir();
        
        /* Create the directory if it doesn't exist*/
        fs::create_dir_all(data_dir)?;

        let file_path = data_dir.join("commands.json");

        /* Check existence, create file if it doesn't exist */
        if !file_path.exists() {
            let initial_data = CommandList {
                commands: Vec::new(),
            };

            let json_data = serde_json::to_string_pretty(&initial_data)?;
            fs::write(&file_path, json_data)?;

            padding(vec![
                "✔ Done!".green().to_string(),
                format!("- commands.json initialized at: {}", file_path.display()),
                "\n👉 Use 'vaulty add <command>' to add a new command.".yellow().to_string(),
            ])
        } else {
            padding(vec![
                "✔ OK!".green().to_string(),
                format!("- commands.json already exists at: {}", file_path.display()),
                "\n👉 Use 'vaulty add <command>' to add a new command.".yellow().to_string(),
            ])
        }
    } else {
        eprintln!("{}", "✕ Unable to find app data directory.".red());
    }

    Ok(())
}
