use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(version = "1.0.0")]
#[command(about = "A simple CLI todo application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a todo, optionally to a project: add [project] <text>
    Add {
        #[arg(required = true, num_args = 1..=2)]
        args: Vec<String>,
    },
    /// Create a new project
    Make {
        proj: String,
    },
    /// List todos, optionally filtered by project
    List {
        proj: Option<String>,
    },
    /// Mark a todo as done by its number
    Done {
        num: i32,
        /// Optional project name
        proj: Option<String>,
    },
}
