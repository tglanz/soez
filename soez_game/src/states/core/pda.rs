pub use super::{
    Transition,
    State,
};

pub struct PDA {
    states: Vec<Box<dyn State>>
}

impl PDA {
    /// creates a new states PDA storage
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            states: vec![initial_state]
        }
    }

    /// mutably peek the top state
    pub fn peek_mut(&mut self) -> &mut Box<dyn State> {
        self.states.last_mut().expect("attempted to `first` on an empty PDA")
    }

    /// peek the top state
    pub fn peek(&self) -> &Box<dyn State> {
        self.states.last().expect("attempted to `first` on an empty PDA")
    }

    /// push a new state to the stack
    /// 
    /// events flow
    /// 1. pause current state
    /// 2. push state, and start it
    /// 3. return whatever transition the state returned
    pub fn push(&mut self, state: Box<dyn State>) -> Transition {
        self.peek_mut().on_pause();
        self.states.push(state);
        self.peek_mut().on_start()
    }

    /// pop the current state
    /// exit it after popping it
    pub fn pop(&mut self) {
        self.states.pop().map(|mut state| state.on_exit());
    }

    /// switch the current, to another state
    /// this is basically a pop and push
    pub fn switch(&mut self, state: Box<dyn State>) -> Transition {
        self.pop();
        self.states.push(state);
        self.peek_mut().on_start()
    }
}