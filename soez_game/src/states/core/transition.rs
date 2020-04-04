pub use super::State;

pub enum Transition {
    None,
    Quit,
    Pop,
    Switch(Box<dyn State>),
    Push(Box<dyn State>),
}
