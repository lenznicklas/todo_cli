use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub id: i32,
    pub next: i32,
}