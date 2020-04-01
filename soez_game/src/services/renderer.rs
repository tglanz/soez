use raylib::prelude::{RaylibDrawHandle, RaylibDraw, Color as RaylibColor};

use crate::prelude::*;
use crate::components::rendering::*;

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
    fn render(&mut self, target: &Rendering, coords: &ScreenCoordinates);
}

impl<'a> RaylibRenderer<'a> {
    pub fn create(draw: RaylibDrawHandle<'a>) -> Self {
        RaylibRenderer { draw }
    }

    fn draw_text(&mut self, coords: &ScreenCoordinates, text: &String, attr: &TextAttributes) {
        let font_size_value = match attr.font_size {
            FontSize::Small => 8,
            FontSize::Medium => 18,
            FontSize::Big => 32,
            FontSize::Huge => 52,
            FontSize::Custom(value) => value,
        };

        self.draw.draw_text(
            &text, 
            coords.x, coords.y,
            font_size_value, 
            attr.color.to_raylib_color()
        )
    }
}

impl<'a> Renderer<'a> for RaylibRenderer<'a> {

    fn clear_background(&mut self, color: &Color) {
        self.draw.clear_background(color.to_raylib_color())
    }

    fn render(&mut self, rendering: &Rendering, coords: &ScreenCoordinates) {
        match &rendering.target {
            RenderingTarget::Circle(circle, attr) => self.draw.draw_circle(
                coords.x as i32, coords.y as i32, circle.radius, attr.color.to_raylib_color()
            ),
            RenderingTarget::Rectangle(rect, attr) => self.draw.draw_rectangle(
                    coords.x as i32, coords.y as i32, rect.width as i32, rect.height as i32,
                    attr.color.to_raylib_color()
            ),
            RenderingTarget::Text(text, attr) => self.draw_text(coords, text, attr)
        }
    }
}