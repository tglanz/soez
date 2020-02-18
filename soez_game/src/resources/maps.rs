use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Map {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Maps {
    maps: Vec<Map>
}