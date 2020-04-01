use specs::*;

use crate::resources::prefabs::*;

use std::iter::FromIterator;

fn with_component_prefab<'a>(builder: EntityBuilder<'a>, component_prefab: ComponentPrefab) -> EntityBuilder<'a> {
    match component_prefab {
        ComponentPrefab::Position(component) => builder.with(component),
        ComponentPrefab::Velocity(component) => builder.with(component),
        ComponentPrefab::Rendering(component) => builder.with(component),
        ComponentPrefab::Acceleration(component) => builder.with(component),
        ComponentPrefab::SingleF32(component) => builder.with(component),
    }
}

pub fn entity(world: &mut World, entity_prefab: EntityPrefab) -> Entity {
    entity_prefab.components_prefabs.into_iter().fold(world.create_entity(), with_component_prefab).build()
}

pub fn enitities(world: &mut World, entities_prefab: EntitiesPrefab) -> Vec<Entity> {
    let manifest = |entity_prefab| entity(world, entity_prefab);
    Vec::from_iter(entities_prefab.entities_prefabs.into_iter().map(manifest))
}