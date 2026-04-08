use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    name: String,
    id: i32,
}

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(version = "1.0")]
#[command(about = "Todo Application")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        text: String,
    },
    List,
    Done {
        num: i32,
    },
}

fn main() {

    let cli = Cli::parse();

    let result = match cli.command {
        None => {
            println!("No arguments were given");
            Ok(())
        }
        Some(Commands::Add{text}) => add_todo(text),
        Some(Commands::List) => list_todo(),
        Some(Commands::Done{num}) => done_todo(num),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    };

}

fn add_todo(text: String) -> io::Result<()> {
    let file = "todo.json";

    let mut todos = load_todos()?;

    let new_id = todos
        .iter()
        .map(|todo| todo.id)
        .max()
        .unwrap_or(0)
        + 1;
    
    let data = Todo {
        name: text,
        id: new_id,
    };

    todos.push(data);

    let json = serde_json::to_string_pretty(&todos)
    .map_err(io::Error::other)?;

    fs::write(file, json)
}

fn load_todos() -> io::Result<Vec<Todo>>{
    let file = "todo.json";

    match fs::read_to_string(file){
        Ok(content) => {
            let todos: Vec<Todo> =
                serde_json::from_str(&content).map_err(io::Error::other)?;
            Ok(todos)
        }
        Err(_) => Ok(Vec::new()),
    }
}

fn list_todo() -> io::Result<()> {
    let todos = load_todos()?;

    if todos.is_empty() {
        println!("No Todos found");
    } else {
        for todo in todos {
            println!("{} | {}", todo.id, todo.name);
        }
    }

    Ok(())
}

fn done_todo(num: i32) -> io::Result<()>{
    let file = "todo.json";
    let mut todos = load_todos()?;

    let original_len = todos.len();
    todos.retain(|todo| todo.id != num);

    if todos.len() == original_len {
        println!("No todo with number {}", num);
    } else {
        let json = serde_json::to_string_pretty(&todos).map_err(io::Error::other)?;
        fs::write(file, json)?;
        println!("Removed todo {}", num);
    }

    Ok(())
}