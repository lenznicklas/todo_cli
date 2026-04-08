use crate::model::Todo;
use std::fs;
use std::io;

const FILE: &str = "todo.json";

pub fn add_todo(text: String) -> io::Result<()> {
    let mut todos = load_todos()?;

    let new_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;

    let data = Todo {
        name: text,
        id: new_id,
    };

    todos.push(data);

    let json = serde_json::to_string_pretty(&todos).map_err(io::Error::other)?;
    fs::write(FILE, json)
}

pub fn list_todo() -> io::Result<()> {
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

pub fn done_todo(num: i32) -> io::Result<()> {
    let mut todos = load_todos()?;

    let original_len = todos.len();
    todos.retain(|todo| todo.id != num);

    if todos.len() == original_len {
        println!("No todo with number {}", num);
    } else {
        let json = serde_json::to_string_pretty(&todos).map_err(io::Error::other)?;
        fs::write(FILE, json)?;
        println!("Removed todo {}", num);
    }

    Ok(())
}

fn load_todos() -> io::Result<Vec<Todo>> {
    match fs::read_to_string(FILE) {
        Ok(content) => {
            let todos: Vec<Todo> =
                serde_json::from_str(&content).map_err(io::Error::other)?;
            Ok(todos)
        }
        Err(_) => Ok(Vec::new()),
    }
}