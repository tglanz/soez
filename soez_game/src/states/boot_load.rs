use std::{
    path::Path,
    time::{Instant, Duration},
};

use crate::prelude::*;

pub struct BootLoadState {
    pub loaded: bool,
    pub start_instant: Instant
}

impl Default for BootLoadState {
    fn default() -> Self {
        Self {
            loaded: false,
            start_instant: Instant::now()
        }
    }
}

impl BootLoadState {
    fn load_maps(&mut self, world: &mut World) {
        let assets_directory = util::get_assets_directory(world);
        let maps_path = Path::new(&assets_directory).join("maps.ron");
        let maps: Maps = util::load_ron_resource(maps_path);
        world.insert(maps);
    }

    fn load_main_menu_prefab(&mut self, world: &mut World) {
        let assets_directory = util::get_assets_directory(world);
        let path = Path::new(&assets_directory).join("prefabs").join("main_menu.ron");
        let prefab: EntitiesPrefab = util::load_ron_resource(path);        

        log::info!("prefab: {:#?}", prefab);
    }
}

impl State for BootLoadState {
    fn get_name(&self) -> &'static str {
        "BootLoadState"
    }


    fn on_update(&mut self, data: &mut StateData) -> Transition {
        if !self.loaded {
            // TODO: async
            self.load_maps(data.world);
            self.load_main_menu_prefab(data.world);
            self.loaded = true;
            Transition::None
        } else if Instant::now() - self.start_instant < Duration::from_secs(1) {
            Transition::None
        } else {
            let map_index = 0;
            Transition::None
//            Transition::Switch(Box::new(LevelLoaderState::new(map_index)))
        }
    }
}