use nalgebra::Vector2;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CoordinateSystem {
    Screen,
    Ndc,
}

pub type ScreenCoordinates = Vector2<i32>;
pub type NdcCoordinates = Vector2<f32>;
