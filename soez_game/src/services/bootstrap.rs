use crate::prelude::*;

pub fn bootstrap_all(world: &mut World) {
    bootstrap_resources(world);
}

pub fn bootstrap_resources(world: &mut World) {
    log::debug!("bootstrapping resources");
    world.insert(Input::default());
    world.insert(EntitiesPrefabsRegistry::new());
}
