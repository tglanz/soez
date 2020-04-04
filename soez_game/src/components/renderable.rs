use serde::Deserialize;
use specs::prelude::*;

use crate::services::geometry;
use crate::services::Color;

#[derive(Debug, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Big,
    Huge,
    Custom(i32),
}

#[derive(Debug, Deserialize)]
pub struct TextAttributes {
    pub color: Color,
    pub font_size: FontSize,
    pub location: geometry::Point,
}

#[derive(Debug, Deserialize)]
pub struct GeometryAttributes {
    pub fill_color: Option<Color>,
    pub border_color: Option<Color>,
}

#[derive(Debug, Deserialize)]
pub enum Renderable {
    Text(String, TextAttributes),
    Geometry(geometry::Geometry, GeometryAttributes),
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
