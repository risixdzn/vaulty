mod commands;
mod types;
mod utils;

use clap::{Arg, Command};

fn main() {
    let mut app = Command::new("vaulty")
    .version("1.0")
    .about("Vaulty - Save your commands, use them later.")
    .subcommand(
        Command::new("add")
            .about("Add a new command to Vaulty.")
            .arg(
                Arg::new("command")
                    .short('c')
                    .long("command")
                    .value_parser(clap::value_parser!(String))
                    .required(true)
                    .help("The command to store")
            )
            .arg(
                Arg::new("description")
                    .short('d')
                    .long("description")
                    .value_parser(clap::value_parser!(String))
                    .required(true)
                    .help("A short description of the command")
            )
    );


    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let command = sub_matches.get_one::<String>("command").expect("Command is required");
            let description = sub_matches.get_one::<String>("description").expect("Description is required");
            commands::add(command, description).expect("Failed to add command");
        },
        _ => app.print_help().expect("Failed to print help"),
    }
}