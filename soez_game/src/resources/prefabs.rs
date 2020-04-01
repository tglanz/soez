use std::vec::Vec;

use serde::Deserialize;

use crate::components::*;

#[derive(Debug, Deserialize)]
pub enum ComponentPrefab {
    Acceleration(Acceleration),
    Position(Position),
    Rendering(Rendering),
    Velocity(Velocity),
    SingleF32(SingleF32)
}

#[derive(Debug, Deserialize)]
pub struct EntityPrefab {
    pub components_prefabs: Vec<ComponentPrefab>,
}

#[derive(Debug, Deserialize)]
pub struct EntitiesPrefab {
    pub entities_prefabs: Vec<EntityPrefab>
}