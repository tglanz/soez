use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize)]
pub struct Circle {
    pub radius: f32,
}