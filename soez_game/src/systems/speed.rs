use specs::prelude::*;

use crate::components::*;

pub struct SpeedSystem;

impl<'a> System<'a> for SpeedSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut position, velocity): Self::SystemData) {
        for (position, velocity) in (&mut position, &velocity).join() {
            position.vector += velocity.vector;
        }
    }
}
