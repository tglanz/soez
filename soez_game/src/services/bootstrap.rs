use std::{
    collections::HashMap
};

use crate::prelude::*;

pub fn bootstrap_all(world: &mut World) {

}

pub fn bootstrap_resources(world: &mut World) {
    let entities_prefabs_registry: HashMap<String, EntitiesPrefab> = HashMap::new();
    world.insert(entities_prefabs_registry);
}