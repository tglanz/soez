use specs::prelude::*;
use serde::Deserialize;
use nalgebra::Vector2;

#[derive(Debug, Deserialize)]
pub struct Velocity {
    pub vector: Vector2<f32>
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y)
        }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}