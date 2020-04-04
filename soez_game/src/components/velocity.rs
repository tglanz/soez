use nalgebra::Vector2;
use serde::Deserialize;
use specs::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Velocity {
    pub vector: Vector2<f32>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y),
        }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
