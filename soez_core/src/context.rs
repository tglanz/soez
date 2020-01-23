use crate::states::{State, Pushdown};

#[derive(Debug)]
struct GameMeta {
    pub name: String,
    pub author: String,
}

impl Default for GameMeta {
    fn default() -> Self {
        Self {
            name: String::from("unknown"),
            author: String::from("unknown"),
        }
    }
}

pub struct Context {
    meta: GameMeta,
    states: Pushdown
}

impl Context {
    pub fn print_stuff(&self) {
        println!("meta: {:#?}", self.meta);
        println!("state: {:#?}", self.states.peek());
    }
}

pub struct ContextBuilder {
    meta: GameMeta,
    initial_state: Box<dyn State>,
}

impl ContextBuilder {
    pub fn create(initial_state: Box<dyn State>) -> Self {
        Self {
            meta: GameMeta::default(),
            initial_state
        }
    }

    pub fn build(self) -> Context {
        Context {
            meta: self.meta,
            states: Pushdown::new(self.initial_state),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.meta.name = String::from(name);
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.meta.author = String::from(author);
        self
    }
}