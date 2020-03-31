use raylib::prelude::{RaylibDrawHandle, RaylibDraw, Color as RaylibColor};
use crate::prelude::*;

trait ToRaylibColor {
    fn to_raylib_color(&self) -> RaylibColor;
}

impl ToRaylibColor for Color {
    fn to_raylib_color(&self) -> RaylibColor {
        RaylibColor::from(self.rgba())
    }
}

pub struct RaylibRenderer<'a> {
    draw: RaylibDrawHandle<'a>
}

pub trait Renderer<'a> {
    fn clear_background(&mut self, color: &Color);
    fn render(&mut self, target: &Rendering, position: &Position);
}

impl<'a> RaylibRenderer<'a> {
    pub fn create(draw: RaylibDrawHandle<'a>) -> Self {
        RaylibRenderer { draw }
    }
}

impl<'a> Renderer<'a> for RaylibRenderer<'a> {

    fn clear_background(&mut self, color: &Color) {
        self.draw.clear_background(color.to_raylib_color())
    }

    fn render(&mut self, rendering: &Rendering, position: &Position) {
        match rendering {
            Rendering::Circle(circle, attr) => self.draw.draw_circle(
                position.vector.x as i32, position.vector.y as i32, circle.radius, attr.color.to_raylib_color()),
            }
    }
}