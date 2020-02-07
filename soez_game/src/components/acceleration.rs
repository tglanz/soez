use specs::prelude::*;
use nalgebra::Vector2;

#[derive(Debug)]
pub struct Acceleration {
    pub vector: Vector2<f32>
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y)
        }
    }
}

impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}