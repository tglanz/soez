use specs::World;

use crate::prelude::*;

pub struct StateData<'a> {
    pub world: &'a mut World
}

impl<'a> StateData<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self {
            world
        }
    }
}

#[allow(unused)]
pub trait State {
    fn get_name(&self) -> &'static str;

    fn on_update(&mut self, data: &mut StateData) -> Transition { Transition::None }

    fn on_start(&mut self, data: &mut StateData) -> Transition { Transition::None }
    fn on_resume(&mut self, data: &mut StateData) -> Transition { Transition::None }

    fn on_pause(&mut self, data: &mut StateData) -> Transition { Transition::None }
    fn on_exit(&mut self, data: &mut StateData)  -> Transition { Transition::None }
}