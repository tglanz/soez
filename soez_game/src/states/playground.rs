use crate::prelude::State;

pub struct PlaygroundState {

}

impl Default for PlaygroundState {
    fn default() -> Self {
        Self {

        }
    }
}

impl State for PlaygroundState {
    fn get_name(&self) -> &'static str {
        return "PlaygroundState";
    }
}