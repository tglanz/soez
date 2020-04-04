use serde::Deserialize;

use crate::prelude::Color;

#[derive(Deserialize)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize)]
pub struct Debug {
    pub enable_backtrace: bool,
}

#[derive(Deserialize)]
pub struct Window {
    pub title: String,
    pub resolution: Resolution,
    pub resizable: bool,
    pub fullscreen: bool,
    pub color: Color,
}

#[derive(Deserialize)]
pub struct Application {
    pub debug: Debug,
    pub assets_directory: String,
    pub window: Window,
}
