use specs::World;

use crate::prelude::*;

pub struct BootLoadState {
    pub loaded: bool,
}

impl Default for BootLoadState {
    fn default() -> Self {
        Self { loaded: false }
    }
}

impl BootLoadState {
    fn load_maps(&mut self, world: &mut World) {
        log::debug!("loading maps");
        let maps_path = util::get_assets_directory(world).join("maps.ron");
        let maps: Maps = util::load_ron_resource(maps_path);
        world.insert(maps);
    }

    fn load_main_menu_prefab(&mut self, world: &mut World) {
        log::debug!("loading main menu prefab");
        let prefab = util::load_prefab(world, "main_menu.ron");

        // update registry
        world
            .fetch_mut::<EntitiesPrefabsRegistry>()
            .insert("main-menu", prefab);
    }
}

impl State for BootLoadState {
    fn get_name(&self) -> &'static str {
        "BootLoadState"
    }

    fn on_update(&mut self, data: &mut StateData) -> Transition {
        if !self.loaded {
            log::debug!("still not fully loaded, loading");
            // TODO: async
            self.load_maps(data.world);
            self.load_main_menu_prefab(data.world);
            self.loaded = true;
            Transition::None
        } else {
            Transition::Switch(Box::new(MainMenuState))
        }
    }
}
