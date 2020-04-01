use std::{
    path::Path,
};
use crate::prelude::*;
use specs::prelude::*;

use level::LevelInfo;

pub struct LevelLoaderState {
    pub map_index: usize,
    pub loaded_map_resource: Option<Map>,
    pub loaded_map_tiled: Option<tiled::Map>,
}

impl LevelLoaderState {
    pub fn new(map_index: usize) -> Self {
        Self {
            loaded_map_resource: None,
            loaded_map_tiled: None,
            map_index
        }
    }

    pub fn load_map(&mut self, world: &World) {
        let maps = world.fetch::<Maps>();
        self.loaded_map_resource = Some(maps.get_cloned(self.map_index));
    }

    pub fn load_tiled_map(&mut self, world: &World) {
        let assets_directory = util::get_assets_directory(world);
        let map = self.loaded_map_resource.as_ref().unwrap();
        let path = Path::new(&assets_directory)
            .join(&map.path);

        let tiled_map = tiled::parse_file(&path)
            .expect(&format!("failed to parse tiled file: {:#?}", path.display()));

        self.loaded_map_tiled = Some(tiled_map);
        
    }
}

impl State for LevelLoaderState {
    fn get_name(&self) -> &'static str {
        "LevelLoaderState"
    }

    fn on_start(&mut self, data: &mut StateData) -> Transition {
        self.load_map(data.world);
        self.load_tiled_map(data.world);
        // self.load_tilesets(data.world);
        Transition::None
    }

    fn on_update(&mut self, _data: &mut StateData) -> Transition {
        
        if self.loaded_map_resource.is_some() && self.loaded_map_tiled.is_some() {
            let map_resource = self.loaded_map_resource.take().unwrap();
            let map_tiled = self.loaded_map_tiled.take().unwrap();

            let info = LevelInfo::new(
                map_resource,
                map_tiled
            );

            let state = LevelState::new(info);
            Transition::Switch(Box::new(state))
        } else {
            Transition::None
        }
    }
}