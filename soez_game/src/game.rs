use log;

use specs::prelude::*;
use crate::prelude::*;

fn initialize_logger(assets_directory: String) {
    let yaml_path = format!("{:#}/log4rs.yaml", assets_directory);
    log4rs::init_file(yaml_path, Default::default()).unwrap();
}

fn create_dispatcher<'a, 'b>(raylib_thread: raylib::RaylibThread) -> Dispatcher<'a, 'b> {
    log::debug!("creating a dispatcher");
    DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(raylib_thread))
        .with(SpeedSystem, "~SpeedSystem", &[])
        .build()
}

fn create_world(dispatcher: &mut Dispatcher, raylib_handle: raylib::RaylibHandle, application: Application) -> World {
    log::debug!("creating a world");
    let mut world = World::new();
    world.insert(RaylibContext::new(raylib_handle));
    world.insert(application);
    dispatcher.setup(&mut world);

    world
}


pub struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn initialize(application: Application) -> Self {

        initialize_logger(application.assets_directory.clone());

        std::panic::set_hook(Box::new(|trace| log::error!("{:#?}", trace) ));
        log::debug!("initializing game");

        let (mut raylib_handle, raylib_thread) = raylib::init()
            .title(&application.title.clone())
            .size(
                application.resolution.width,
                application.resolution.height)
            .build();

        let mut dispatcher = create_dispatcher(raylib_thread);
        let world = create_world(&mut dispatcher, raylib_handle, application);

        Self { dispatcher, world }
    }

    pub fn should_close(&mut self) -> bool {
        // add logic accordigly
        // 1. user requested to exit for example
        self.world.fetch::<RaylibContext>().handle.window_should_close()
    }

    pub fn update(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}