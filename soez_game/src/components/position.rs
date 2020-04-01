use specs::prelude::*;
use serde::Deserialize;
use nalgebra::Vector2;

#[derive(Debug, Deserialize)]
pub struct Position {
    pub vector: Vector2<f32>
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vector: Vector2::new(x, y)
        }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}