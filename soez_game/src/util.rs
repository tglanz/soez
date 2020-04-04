use crate::prelude::*;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

pub fn load_yaml_resource<D, P>(path: P) -> D
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    let content = std::fs::read_to_string(path.as_ref().clone()).expect(&format!(
        "failed to read file at: {:#?}",
        path.as_ref().display()
    ));
    serde_yaml::from_str(&content).expect(&format!(
        "failed to deserialize file at: {:#?}",
        path.as_ref().display()
    ))
}

pub fn load_ron_resource<D, P>(path: P) -> D
where
    D: DeserializeOwned,
    P: AsRef<Path>,
{
    let content = std::fs::read_to_string(path.as_ref().clone()).expect(&format!(
        "failed to read file at: {:#?}",
        path.as_ref().display()
    ));
    ron::de::from_str(&content).expect(&format!(
        "failed to deserialize file at: {:#?}",
        path.as_ref().display()
    ))
}

pub fn load_prefab<S>(world: &World, name: S) -> EntitiesPrefab
where
    S: AsRef<str>,
{
    let prefab_path = get_prefabs_directory(world).join(name.as_ref());
    load_ron_resource(prefab_path)
}

// TODO: cachable
pub fn get_assets_directory(world: &World) -> PathBuf {
    let s: String = world.fetch::<Application>().assets_directory.clone();
    Path::new(&s).to_path_buf()
}

// TODO: cachable
pub fn get_prefabs_directory(world: &World) -> PathBuf {
    get_assets_directory(world).join("prefabs").to_path_buf()
}
