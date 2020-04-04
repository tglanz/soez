use nalgebra::Vector2;
use serde::Deserialize;

pub type Scalar = f32;
pub type Point = Vector2<Scalar>;
pub type Size = Vector2<Scalar>;

#[derive(Debug, Deserialize)]
pub enum Geometry {
    Rectangle { center: Point, size: Size },
    Circle { center: Point, radius: Scalar },
}
