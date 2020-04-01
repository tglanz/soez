use serde::Deserialize;
use nalgebra::Vector2;

#[derive(Debug, Deserialize)]
pub enum CoordinateSystem {
    Screen,
    Ndc
}

pub type ScreenCoordinates = Vector2<i32>;
pub type NdcCoordinates = Vector2<f32>;