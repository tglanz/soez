extern crate log;
extern crate log4rs;
extern crate raylib;

pub mod states;
pub mod components;
pub mod systems;
pub mod resources;
pub mod prelude;

use specs::prelude::*;
use prelude::*;

#[derive(Debug)]
pub struct OnBoardingState;

impl State for OnBoardingState {
    fn get_name(&self) -> &'static str {
        "OnBoardingState"
    }
}

fn initialize_logger() {
    log4rs::init_file("resources/log4rs.yaml", Default::default()).unwrap();
}

fn create_dispatcher<'a, 'b>(raylib: RaylibContext) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(raylib))
        .with(SpeedSystem, "~SpeedSystem", &[])
        .build()
}

fn create_world(dispatcher: &mut Dispatcher) -> World {
    let mut world = World::new();
    world.insert::<Application>(Default::default());
    dispatcher.setup(&mut world);

    world.register::<Position>();

    world
}

fn main() {

    initialize_logger();

    std::panic::set_hook(Box::new(|trace| log::error!("{:#?}", trace) ));
    log::info!("start");

    // let raylibContext = RaylibContext::create(RaylibBuilder {
    //     width: 640,
    //     height: 480,
    //     title: "raylib-rs".to_string(),
    //     ..Default::default()
    // });

    let mut dispatcher = create_dispatcher(Default::default());
    let mut world = create_world(&mut dispatcher);

    world.create_entity()
        .with(Position::new(0.0, 0.0))
        .with(Circle::new(10.0, 0))
        .with(Velocity::new(0.1, 0.1))
        .build();

    while !world.fetch::<Application>().window_should_close {
        dispatcher.dispatch(&world);
    }
    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);
    //     dispatcher.dispatch_par(&world);

    //     d.clear_background(Color::WHITE);
    //     // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    // }
}