mod cli;
mod model;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        None => {
            println!("No arguments were given");
            Ok(())
        }
        Some(Commands::Add { text }) => todo::add_todo(text),
        Some(Commands::List) => todo::list_todo(),
        Some(Commands::Done { num }) => todo::done_todo(num),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}