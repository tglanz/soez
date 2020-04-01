use specs::prelude::*;
use serde::Deserialize;
use crate::prelude::Color;
use crate::geometry;

#[derive(Debug, Deserialize)]
pub struct GeometryAttributes {
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct TextAttributes {
    pub color: Color,
    pub font_size: f32,
}

#[derive(Debug, Deserialize)]
pub enum Rendering {
    Circle(geometry::Circle, GeometryAttributes),
    Rectangle(geometry::Rectangle, GeometryAttributes),
    Text(String, TextAttributes),
}

impl Component for Rendering {
    type Storage = VecStorage<Self>;
}