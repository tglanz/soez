use crate::prelude::Color;

#[derive(Debug)]
pub struct Application {
    pub clear_color: Color,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            clear_color: Color::AliceBlue,
        }
    }
}