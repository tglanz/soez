use serde::{Deserialize};

use crate::prelude::Color;

#[derive(Deserialize)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize)]
pub struct Application {
    pub title: String,
    pub resolution: Resolution,
    pub assets_directory: String,
    pub clear_color: Color,
}