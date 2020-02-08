extern crate log;
extern crate log4rs;
extern crate raylib;

pub mod services;
pub mod states;
pub mod components;
pub mod systems;
pub mod resources;
pub mod prelude;

use specs::prelude::*;
use crate::prelude::*;

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

fn create_dispatcher<'a, 'b>(raylib_thread: raylib::RaylibThread) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(raylib_thread))
        .with(SpeedSystem, "~SpeedSystem", &[])
        .build()
}

fn create_world(dispatcher: &mut Dispatcher, raylib_handle: raylib::RaylibHandle) -> World {
    let mut world = World::new();
    world.insert(RaylibContext::new(raylib_handle));
    world.insert(Application::default());
    dispatcher.setup(&mut world);

    world.register::<Position>();
    world.register::<Velocity>();

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

    let (raylib_handle, raylib_thread) = raylib::init().build();

    let mut dispatcher = create_dispatcher(raylib_thread);
    let mut world = create_world(&mut dispatcher, raylib_handle);

    world.create_entity()
        .with(Position::new(0.0, 0.0))
        .with(Velocity::new(0.1, 0.1))
        .with(Rendering::Circle(10.0, Color::Red))
        .build();

    while !world.fetch::<RaylibContext>().handle.window_should_close() {
        dispatcher.dispatch(&world);
    }
    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);
    //     dispatcher.dispatch_par(&world);

    //     d.clear_background(Color::WHITE);
    //     // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    // }
}