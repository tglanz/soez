use nalgebra::Vector2;
use serde::Deserialize;
use specs::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Acceleration {
    pub vector: Vector2<f32>,
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y),
        }
    }
}

impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}
