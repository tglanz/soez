use std::path::Path;
use crate::prelude::*;
use serde::de::DeserializeOwned;

pub fn load_yaml_resource<D, P>(path: P) -> D
where
    D: DeserializeOwned,
    P: AsRef<Path>
{
    let content = std::fs::read_to_string(path.as_ref().clone())
        .expect(&format!("failed to read file at: {:#?}", path.as_ref().display()));
    serde_yaml::from_str(&content)
        .expect(&format!("failed to deserialize file at: {:#?}", path.as_ref().display()))
}

pub fn load_ron_resource<D, P>(path: P) -> D
where
    D: DeserializeOwned,
    P: AsRef<Path>
{
    let content = std::fs::read_to_string(path.as_ref().clone())
        .expect(&format!("failed to read file at: {:#?}", path.as_ref().display()));
    ron::de::from_str(&content)
        .expect(&format!("failed to deserialize file at: {:#?}", path.as_ref().display()))
}

pub fn get_assets_directory(world: &World) -> String {
    world.fetch::<Application>().assets_directory.clone()
}