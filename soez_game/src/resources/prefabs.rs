use std::collections::HashMap;
use std::vec::Vec;

use serde::Deserialize;

use crate::components::*;

#[derive(Debug, Deserialize)]
pub enum ComponentPrefab {
    Renderable(Renderable),
    Acceleration(Acceleration),
    Position(Position),
    Velocity(Velocity),
    SingleF32(SingleF32),
}

#[derive(Debug, Deserialize)]
pub struct EntityPrefab {
    pub components_prefabs: Vec<ComponentPrefab>,
}

#[derive(Debug, Deserialize)]
pub struct EntitiesPrefab {
    pub entities_prefabs: Vec<EntityPrefab>,
}

pub type EntitiesPrefabsRegistry = HashMap<&'static str, EntitiesPrefab>;
