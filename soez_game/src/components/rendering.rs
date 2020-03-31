use specs::prelude::*;
use crate::prelude::Color;
use crate::geometry;

pub struct Attributes {
    pub color: Color,
}

pub enum Rendering {
    Circle(geometry::Circle, Attributes),
    Rectangle(geometry::Rectangle, Attributes)
}

impl Component for Rendering {
    type Storage = VecStorage<Self>;
}