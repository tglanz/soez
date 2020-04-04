use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Map {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Maps {
    maps: Vec<Map>,
}

impl Maps {
    pub fn len(&self) -> usize {
        self.maps.len()
    }

    pub fn get_cloned(&self, index: usize) -> Map {
        self.maps.get(index).unwrap().clone()
    }
}
