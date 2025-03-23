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
            Command::new("add")
                .about("Add a new command to Vaulty.")
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
        );

    let matches = app.clone().get_matches();

    /*
        For most part of the commands:
        - If parameters are passed, it will add the command to the list.
        - If not, it will prompt the user for input.
    */
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            if
                let (Some(command), Some(description)) = (
                    sub_matches.get_one::<String>("command"),
                    sub_matches.get_one::<String>("description"),
                )
            {
                commands::add(command, description).expect("Failed to add command");
            } else {
                commands::interactive_add().expect("Failed to add command interactively");
            }
        }
        _ => app.print_help().expect("Failed to print help"),
    }
}
