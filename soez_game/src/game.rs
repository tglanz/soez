use crate::prelude::*;
use crate::states::core::Pda;
use backtrace::Backtrace;
use log;
use specs::prelude::*;

fn initialize_logger(assets_directory: String) {
    let yaml_path = format!("{:#}/log4rs.yaml", assets_directory);
    log4rs::init_file(yaml_path, Default::default()).unwrap();
}

fn create_dispatcher<'a, 'b>(raylib_thread: raylib::RaylibThread) -> Dispatcher<'a, 'b> {
    log::debug!("creating a dispatcher");
    DispatcherBuilder::new()
        .with_thread_local(RenderSystem::new(raylib_thread))
        .with(InputSystem, "~InputSystem", &[])
        .with(SpeedSystem, "~SpeedSystem", &["~InputSystem"])
        .build()
}

fn create_world(
    dispatcher: &mut Dispatcher,
    raylib_handle: raylib::RaylibHandle,
    application: Application,
) -> World {
    log::debug!("creating a world");
    let mut world = World::new();
    world.insert(RaylibContext::new(raylib_handle));
    world.insert(application);
    world.insert(Signals::default());
    dispatcher.setup(&mut world);

    world
}

fn initialize_panic_hook(enable_backtrace: bool) {
    std::panic::set_hook(Box::new(move |trace| {
        if enable_backtrace {
            log::error!("oh no, {:#?}, {:#?}", trace, Backtrace::new());
        } else {
            log::error!("oh no, {:#?}", trace);
        }
    }));
}

fn create_raylib(application: &Application) -> (raylib::RaylibHandle, raylib::RaylibThread) {
    let mut root = raylib::init();

    let builder = root.title(&application.window.title.clone()).size(
        application.window.resolution.width,
        application.window.resolution.height,
    );

    if application.window.fullscreen {
        builder.fullscreen();
    }

    if application.window.resizable {
        builder.resizable();
    }

    builder.build()
}

pub struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    pda: Pda,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn initialize(application: Application) -> Self {
        initialize_logger(application.assets_directory.clone());
        initialize_panic_hook(application.debug.enable_backtrace);

        log::debug!("initializing game");

        let (raylib_handle, raylib_thread) = create_raylib(&application);

        let mut dispatcher = create_dispatcher(raylib_thread);
        let mut world = create_world(&mut dispatcher, raylib_handle, application);
        let pda = Pda::new(Box::new(BootLoadState::default()));

        bootstrap::bootstrap_all(&mut world);

        Self {
            dispatcher,
            world,
            pda,
        }
    }

    pub fn should_close(&mut self) -> bool {
        // add logic accordigly
        // 1. user requested to exit for example
        // 2. check signals for a pending quit request
        self.world
            .fetch::<RaylibContext>()
            .handle
            .window_should_close()
            || self.world.fetch::<Signals>().pending_quit
    }

    pub fn update(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.pda.update(&mut self.world);
    }
}
