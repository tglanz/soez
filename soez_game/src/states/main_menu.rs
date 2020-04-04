use crate::prelude::*;

pub struct MainMenuState;

impl State for MainMenuState {
    fn get_name(&self) -> &'static str {
        return "MainMenuState";
    }

    fn on_start(&mut self, data: &mut StateData) -> Transition {
        prefabs_manifestation::extract_from_registry(data.world, "main-menu");
        Transition::None
    }
}
