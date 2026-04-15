mod cli;
mod model;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    if let Err(err) = todo::ensure_dir() {
        eprintln!("Error creating data directory: {err}");
        return;
    }

    let cli = Cli::parse();

    let result = match cli.command {
        None => {
            println!("No command given. Run with --help for usage.");
            Ok(())
        }
        Some(Commands::Add { args }) => match args.as_slice() {
            [text] => todo::add_todo(text.clone(), None),
            [proj, text] => todo::add_todo(text.clone(), Some(proj.clone())),
            _ => unreachable!("clap enforces 1..=2 args"),
        },
        Some(Commands::Make { proj }) => todo::make_project(&proj),
        Some(Commands::List { proj }) => todo::list_todo(&proj),
        Some(Commands::Done { num, proj }) => todo::done_todo(num, proj),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }
}
