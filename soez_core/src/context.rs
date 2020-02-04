use specs::{Dispatcher, DispatcherBuilder, World};

use crate::states::{State, Pushdown};

pub struct Context<'a, 'b> {
    states: Pushdown,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

pub struct ContextBuilder<'a, 'b> {
    initial_state: Box<dyn State>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> ContextBuilder<'a, 'b> {
    pub fn create(initial_state: Box<dyn State>) -> Self {
        Self {
            initial_state,
            dispatcher: None,
        }
    }

    pub fn build(self) -> Context<'a, 'b> {
        Context {
            states: Pushdown::new(self.initial_state),
            dispatcher: self.dispatcher.unwrap_or(DispatcherBuilder::new().build()),
            world: World::empty(),
        }
    }

    pub fn with_dispatcher(mut self, dispatcher: Dispatcher<'a, 'b>) -> Self {
        self.dispatcher = Some(dispatcher);
        self
    }
}