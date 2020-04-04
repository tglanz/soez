use raylib::prelude::{Color as RaylibColor, RaylibDraw, RaylibDrawHandle};

use crate::components::renderable::*;
use crate::prelude::*;
use crate::services::geometry::*;

trait ToRaylibColor {
    fn to_raylib_color(&self) -> RaylibColor;
}

impl ToRaylibColor for Color {
    fn to_raylib_color(&self) -> RaylibColor {
        RaylibColor::from(self.rgba())
    }
}

pub struct RaylibRenderer<'a> {
    draw: RaylibDrawHandle<'a>,
}

pub trait Renderer<'a> {
    fn clear_background(&mut self, color: &Color);
    fn render(&mut self, renderable: &Renderable);
}

impl<'a> RaylibRenderer<'a> {
    pub fn create(draw: RaylibDrawHandle<'a>) -> Self {
        RaylibRenderer { draw }
    }

    fn draw_text(&mut self, text: &String, attr: &TextAttributes) {
        let font_size_value = match attr.font_size {
            FontSize::Small => 8,
            FontSize::Medium => 18,
            FontSize::Big => 32,
            FontSize::Huge => 52,
            FontSize::Custom(value) => value,
        };

        self.draw.draw_text(
            &text,
            attr.location.x as i32,
            attr.location.y as i32,
            font_size_value,
            attr.color.to_raylib_color(),
        )
    }

    fn draw_geometry(&mut self, geo: &Geometry, attr: &GeometryAttributes) {
        match geo {
            Geometry::Circle { center, radius } => {
                if let Some(color) = &attr.fill_color {
                    self.draw.draw_circle(
                        center.x as i32,
                        center.y as i32,
                        *radius,
                        color.to_raylib_color(),
                    );
                }
            }
            Geometry::Rectangle { center, size } => {
                if let Some(color) = &attr.fill_color {
                    self.draw.draw_rectangle(
                        center.x as i32,
                        center.y as i32,
                        size.x as i32,
                        size.y as i32,
                        color.to_raylib_color(),
                    );
                }

                if let Some(color) = &attr.border_color {
                    self.draw.draw_rectangle_lines(
                        center.x as i32,
                        center.y as i32,
                        size.x as i32,
                        size.y as i32,
                        color.to_raylib_color(),
                    );
                }
            }
        }
    }
}

impl<'a> Renderer<'a> for RaylibRenderer<'a> {
    fn clear_background(&mut self, color: &Color) {
        self.draw.clear_background(color.to_raylib_color())
    }

    fn render(&mut self, renderable: &Renderable) {
        match renderable {
            Renderable::Text(text, attr) => self.draw_text(text, attr),
            Renderable::Geometry(geo, attr) => self.draw_geometry(geo, attr),
        }
    }
}
