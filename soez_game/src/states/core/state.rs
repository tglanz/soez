use super::{
    Transition
};

pub trait State {
    fn get_name(&self) -> &'static str;

    fn on_update(&mut self) -> Transition { Transition::None }

    fn on_start(&mut self) -> Transition { Transition::None }
    fn on_exit(&mut self) {}
    fn on_resume(&mut self) -> Transition { Transition::None }
    fn on_pause(&mut self) {}
}