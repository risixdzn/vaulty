mod commands;
mod types;
mod utils;

use clap::{ Arg, Command };
use colored::*;

fn main() {
    let mut app = Command::new("vaulty")
        .version("1.0")
        .about("\nVaulty - Save your commands, use them later.".cyan().to_string())
        .subcommand(
            Command::new("save")
                .about("Save a command to Vaulty.")
                .arg(
                    Arg::new("command")
                        .short('c')
                        .long("command")
                        .value_parser(clap::value_parser!(String))
                        .help("The command to store")
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .value_parser(clap::value_parser!(String))
                        .help("A short description of the command")
                )
        )
        .subcommand(Command::new("list").about("List all the commands stored in Vaulty."));

    let matches = app.clone().get_matches();

    /*
        For most part of the commands:
        - If parameters are passed, it will add the command to the list.
        - If not, it will prompt the user for input.
    */
    match matches.subcommand() {
        Some(("save", sub_matches)) => {
            if
                let (Some(command), Some(description)) = (
                    sub_matches.get_one::<String>("command"),
                    sub_matches.get_one::<String>("description"),
                )
            {
                commands::save(command, description).expect("Failed to add command");
            } else {
                commands::interactive_save().expect("Failed to add command interactively");
            }
        }
        Some(("list", _)) => commands::list().expect("Failed to list commands"),
        _ => app.print_help().expect("Failed to print help"),
    }
}
