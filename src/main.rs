mod commands;
mod types;
mod utils;

use clap::Command;

fn main() {
    let mut app = Command::new("vaulty")
        .version("1.0")
        .about("Vaulty - Save your commands, use them later.")
        .subcommand(Command::new("init").about("Initalize Vaulty, creating storage files."));
    
    let matches = app.clone().get_matches();
    
    match matches.subcommand() {
        Some(("init", _)) => commands::init().expect("Failed to initialize commands.json"),
        _ => app.print_help().expect("Failed to print help"),
    }
}