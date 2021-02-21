use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Deserialize, Serialize)]
pub(crate) struct Todo {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) completed: bool,
}

#[derive(Default)]
pub(crate) struct Database {
    pub(crate) store: RwLock<HashMap<u32, Todo>>,
}
