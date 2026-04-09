use crate::model::Todo;
use std::fs;
use std::io;

const FILE: &str = "todo.json";

pub fn add_todo(text: String) -> io::Result<()> {
    let mut todos = load_todos()?;

    todos.push(Todo {
        name: text,
        id: 0,
        next: -1,
    });

    normalize_todos(&mut todos);
    save_todos(&todos)
}

pub fn list_todo() -> io::Result<()> {
    let mut todos = load_todos()?;
    normalize_todos(&mut todos);

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
        return Ok(());
    }

    normalize_todos(&mut todos);
    save_todos(&todos)?;
    println!("Removed todo {}", num);

    Ok(())
}

fn load_todos() -> io::Result<Vec<Todo>> {
    match fs::read_to_string(FILE) {
        Ok(content) => {
            let todos: Vec<Todo> = serde_json::from_str(&content).map_err(io::Error::other)?;
            Ok(todos)
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(err) => Err(err),
    }
}

fn save_todos(todos: &[Todo]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos).map_err(io::Error::other)?;
    fs::write(FILE, json)
}

fn normalize_todos(todos: &mut [Todo]) {
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.id = (index as i32) + 1;
    }

    let ids: Vec<i32> = todos.iter().map(|todo| todo.id).collect();

    for (index, todo) in todos.iter_mut().enumerate() {
        todo.next = ids.get(index + 1).copied().unwrap_or(-1);
    }
}
