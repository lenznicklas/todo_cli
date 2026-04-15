use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    /// ID of the next todo in the list, or -1 if last.
    /// Computed on load/save — not stored persistently.
    #[serde(skip_serializing)]
    #[serde(default = "default_next")]
    pub next: i32,
}

fn default_next() -> i32 {
    -1
}
