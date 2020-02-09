pub use super::{
    State
};

pub enum Transition {
    None,
    Pop,
    Switch(Box<dyn State>),
    Push(Box<dyn State>),
}