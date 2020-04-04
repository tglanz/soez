use specs::*;

use crate::resources::prefabs::*;

use std::iter::FromIterator;

fn with_component_prefab<'a>(
    builder: EntityBuilder<'a>,
    component_prefab: ComponentPrefab,
) -> EntityBuilder<'a> {
    match component_prefab {
        ComponentPrefab::Position(component) => builder.with(component),
        ComponentPrefab::Velocity(component) => builder.with(component),
        ComponentPrefab::Renderable(component) => builder.with(component),
        ComponentPrefab::Acceleration(component) => builder.with(component),
        ComponentPrefab::SingleF32(component) => builder.with(component),
    }
}

pub fn entity(world: &mut World, entity_prefab: EntityPrefab) -> Entity {
    entity_prefab
        .components_prefabs
        .into_iter()
        .fold(world.create_entity(), with_component_prefab)
        .build()
}

pub fn enitities(world: &mut World, entities_prefab: EntitiesPrefab) -> Vec<Entity> {
    let manifest = |entity_prefab| entity(world, entity_prefab);
    Vec::from_iter(entities_prefab.entities_prefabs.into_iter().map(manifest))
}

pub fn extract_from_registry<S: AsRef<str>>(world: &mut World, name: S) -> Vec<Entity> {
    log::debug!(
        "extracting and manifesting from registry: {:#?}",
        name.as_ref()
    );
    let prefab = world
        .fetch_mut::<EntitiesPrefabsRegistry>()
        .remove(name.as_ref())
        .unwrap();
    enitities(world, prefab)
}
