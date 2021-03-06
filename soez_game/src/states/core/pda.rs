pub use specs::World;

use crate::prelude::*;

pub use super::{State, Transition};

pub struct Pda {
    started_initial_state: bool,
    states: Vec<Box<dyn State>>,
}

impl Pda {
    /// creates a new states Pda storage
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            started_initial_state: false,
            states: vec![initial_state],
        }
    }

    /// mutably peek the top state
    pub fn peek_mut(&mut self) -> &mut Box<dyn State> {
        self.states
            .last_mut()
            .expect("attempted to `first` on an empty pda")
    }

    /// peek the top state
    pub fn peek(&self) -> &Box<dyn State> {
        self.states
            .last()
            .expect("attempted to `first` on an empty pda")
    }

    /// push a new state to the stack
    ///
    /// events flow
    /// 1. pause current state
    /// 2. push state, and start it
    /// 3. return whatever transition the state returned
    pub fn push(&mut self, state: Box<dyn State>, data: &mut StateData) -> Transition {
        if self.states.len() == 0 {
            log::debug!("push: skipping pause, no states");
        } else {
            log::debug!("push: pausing state: {:#?}", self.peek().get_name());
            self.peek_mut().on_pause(data);
        }

        log::debug!("push: pushing state: {:#?}", state.get_name());
        self.states.push(state);
        log::debug!("push: starting state: {:#?}", self.peek().get_name());
        self.peek_mut().on_start(data)
    }

    /// pop the current state
    /// exit it after popping it
    pub fn pop(&mut self, data: &mut StateData) {
        log::debug!("pop - popping state: {:#?}", self.peek().get_name());
        self.states.pop().map(|mut state| {
            log::debug!("pop - exiting state: {:#?}", state.get_name());
            state.on_exit(data)
        });
    }

    /// switch the current, to another state
    /// this is basically a pop and push
    pub fn switch(&mut self, state: Box<dyn State>, data: &mut StateData) -> Transition {
        log::debug!("switch - popping");
        self.pop(data);
        log::debug!("switch - pushing");
        self.push(state, data)
    }

    pub fn update(&mut self, world: &mut World) {
        let data = &mut StateData::new(world);

        let mut next_transition = if !self.started_initial_state {
            self.started_initial_state = true;
            Some(self.peek_mut().on_start(data))
        } else {
            Some(self.peek_mut().on_update(data))
        };

        while let Some(transition) = next_transition {
            next_transition = match transition {
                Transition::Pop => {
                    self.pop(data);
                    None
                }
                Transition::Push(new_state) => Some(self.push(new_state, data)),
                Transition::Switch(new_state) => Some(self.switch(new_state, data)),
                Transition::Quit => {
                    data.world.fetch_mut::<Signals>().pending_quit = true;
                    None
                }
                Transition::None => None,
            }
        }
    }
}
