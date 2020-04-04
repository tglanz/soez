use nalgebra::Vector2;
use serde::Deserialize;
use specs::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Position {
    pub vector: Vector2<f32>,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y),
        }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
