use specs::prelude::*;
use serde::Deserialize;

use crate::services::Color;
use crate::services::geometry;
use crate::services::coordinates::CoordinateSystem;

#[derive(Debug, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Big,
    Huge,
    Custom(i32)
}

#[derive(Debug, Deserialize)]
pub struct TextAttributes {
    pub color: Color,
    pub font_size: FontSize,
}

#[derive(Debug, Deserialize)]
pub struct GeometryAttributes {
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub enum RenderingTarget {

    // Textual Targets
    Text(String, TextAttributes),

    // Geometric Targets
    Circle(geometry::Circle, GeometryAttributes),
    Rectangle(geometry::Rectangle, GeometryAttributes), 
}

#[derive(Debug, Deserialize)]
pub struct Rendering {
    pub target: RenderingTarget,
    pub coordinate_system: Option<CoordinateSystem>
}

impl Component for Rendering {
    type Storage = VecStorage<Self>;
}