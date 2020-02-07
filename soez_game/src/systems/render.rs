use specs::prelude::*;
use raylib::prelude::*;
use crate::prelude::*;

pub struct RaylibContext {
    handle: RaylibHandle,
    thread: RaylibThread,
}

impl RaylibContext {
    pub fn create(raylibBuilder: RaylibBuilder) -> Self {
        let (handle, thread) = raylibBuilder.build();
        Self { handle, thread }
    }
}

impl Default for RaylibContext {
    fn default() -> Self {
        RaylibContext::create(raylib::init())
    }
}

pub struct RenderSystem {
    raylib: RaylibContext,
}

impl RenderSystem {
    pub fn new(raylib: RaylibContext) -> Self {
        Self { raylib }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, Application>,

        ReadStorage<'a, Position>,
        ReadStorage<'a, Circle>,
    );

    fn run(&mut self, (mut application, positions, circles): Self::SystemData) {
        if self.raylib.handle.window_should_close() {
            application.window_should_close = true;
            return;
        }

        let mut d = self.raylib.handle.begin_drawing(&self.raylib.thread);
        d.clear_background(Color::WHITE);

        for (position, circle) in (&positions, &circles).join() {
            d.draw_circle(position.vector.x as i32, position.vector.y as i32, circle.radius, raylib::core::color::Color::RED);
        }

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}