use crate::prelude::State;

pub struct MainMenuState {

}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {

        }
    }
}

impl State for MainMenuState {
    fn get_name(&self) -> &'static str {
        return "MainMenuState";
    }
}