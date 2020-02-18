use crate::prelude::*;

pub struct OnBoardState;

impl State for OnBoardState {
    fn get_name(&self) -> &'static str {
        "OnBoardState"
    }

    fn on_update(&mut self) -> Transition {
        Transition::Switch(Box::new(BootLoadState::default()))
    }
}