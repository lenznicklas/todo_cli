use serde::{Deserialize, Serialize};

fn default_next() -> i32 {
    -1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub id: i32,
    #[serde(default = "default_next")]
    pub next: i32,
}
