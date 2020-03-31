use crate::prelude::*;

pub struct LevelInfo {
    pub map_resource: Map,
    pub map_tiled: tiled::Map,
}

impl LevelInfo {
    pub fn new(map_resource: Map, map_tiled: tiled::Map) -> Self {
        Self { map_resource, map_tiled }
    }
}

pub struct LevelState {
    pub info: LevelInfo
}

impl LevelState {
    pub fn new(info: LevelInfo) -> Self {
        Self {
            info
        }
    }

}

impl State for LevelState {
    fn get_name(&self) -> &'static str {
        "LevelState"
    }

    fn on_start(&mut self, _data: &mut StateData) -> Transition {
        log::info!("{:#?}", self.info.map_tiled);
        Transition::None
    }
}