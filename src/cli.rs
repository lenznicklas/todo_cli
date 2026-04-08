use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(version = "1.0")]
#[command(about = "Todo Application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        text: String,
    },
    List,
    Done {
        num: i32,
    },
}