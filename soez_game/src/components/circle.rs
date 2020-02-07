use specs::prelude::*;
use nalgebra::Vector2;

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
    pub color: u32
}

impl Circle {
    pub fn new(radius: f32, color: u32) -> Self {
        Self { radius, color }
    }
}

impl Component for Circle {
    type Storage = VecStorage<Self>;
}