extern crate raylib;
extern crate soez_core;

use raylib::prelude::*;
use soez_core::prelude::*;
use soez_core::specs::prelude::*;

#[derive(Debug)]
pub struct OnBoardingState;

impl State for OnBoardingState {
    fn get_name(&self) -> &'static str {
        "OnBoardingState"
    }
}

fn main() {

    let dispatcher = DispatcherBuilder::new()
        .build();

    let mut context = ContextBuilder::create(Box::new(OnBoardingState))
        .with_dispatcher(dispatcher)
        .build();

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    
    while !rl.window_should_close() {
        let world = context.get_world_mut();
        
        let mut d = rl.begin_drawing(&thread);
    
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}