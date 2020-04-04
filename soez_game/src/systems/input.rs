use crate::prelude::*;
use specs::prelude::*;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadExpect<'a, RaylibContext>, WriteExpect<'a, Input>);

    fn run(&mut self, (raylib_context, _input): Self::SystemData) {
        let _handle = &raylib_context.handle;
        // TODO: stuff
    }
}
