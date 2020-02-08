use specs::prelude::*;
use crate::prelude::Color;

pub enum Rendering {
    Circle(f32, Color)
}

impl Component for Rendering {
    type Storage = VecStorage<Self>;
}