use crate::model::Todo;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

fn data_dir() -> io::Result<PathBuf> {
    let home = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME environment variable is not set"))?;

    Ok(PathBuf::from(home).join(".local/share/todo"))
}

/// Returns the path for a named project, or the default todo file.
fn resolve_file(proj: Option<&str>) -> io::Result<PathBuf> {
    match proj {
        Some(name) => Ok(data_dir()?.join(format!("{name}.json"))),
        None => Ok(data_dir()?.join("todo.json")),
    }
}

// ---------------------------------------------------------------------------
// Public commands
// ---------------------------------------------------------------------------

pub fn ensure_dir() -> io::Result<()> {
    fs::create_dir_all(data_dir()?)?;
    Ok(())
}

pub fn make_project(proj: &str) -> io::Result<()> {
    let path = resolve_file(Some(proj))?;

    if path.exists() {
        println!("Project '{proj}' already exists");
    } else {
        save_todos(&[], &path)?;
        println!("Created project '{proj}'");
    }

    Ok(())
}

pub fn add_todo(text: String, proj: Option<String>) -> io::Result<()> {
    let path = resolve_file(proj.as_deref())?;
    let mut todos = load_todos(&path)?;

    todos.push(Todo {
        id: 0,   // will be set by normalize_todos
        name: text,
        next: -1,
    });

    normalize_todos(&mut todos);
    save_todos(&todos, &path)
}

pub fn list_todo(proj: &Option<String>) -> io::Result<()> {
    let path = resolve_file(proj.as_deref())?;
    let mut todos = load_todos(&path)?;
    normalize_todos(&mut todos);

    if todos.is_empty() {
        println!("No todos found");
    } else {
        for todo in &todos {
            println!("{:>3} | {}", todo.id, todo.name);
        }
    }

    Ok(())
}

pub fn done_todo(num: i32, proj: Option<String>) -> io::Result<()> {
    let path = resolve_file(proj.as_deref())?;
    let mut todos = load_todos(&path)?;

    let original_len = todos.len();
    todos.retain(|todo| todo.id != num);

    if todos.len() == original_len {
        println!("No todo with number {num}");
        return Ok(());
    }

    normalize_todos(&mut todos);
    save_todos(&todos, &path)?;
    println!("Completed todo {num}");

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn load_todos(file: &PathBuf) -> io::Result<Vec<Todo>> {
    match fs::read_to_string(file) {
        Ok(content) => {
            serde_json::from_str(&content).map_err(io::Error::other)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

fn save_todos(todos: &[Todo], file: &PathBuf) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos).map_err(io::Error::other)?;
    fs::write(file, json)
}

/// Re-assigns sequential IDs (1-based) and updates `next` pointers.
fn normalize_todos(todos: &mut [Todo]) {
    for (i, todo) in todos.iter_mut().enumerate() {
        todo.id = (i as i32) + 1;
    }
    let len = todos.len() as i32;
    for todo in todos.iter_mut() {
        todo.next = if todo.id < len { todo.id + 1 } else { -1 };
    }
}
