use specs::prelude::*;

use raylib::{
    RaylibThread,
};

use crate::prelude::*;

pub struct RenderSystem {
    raylib_thread: RaylibThread,
}

impl RenderSystem {
    pub fn new(raylib_thread: RaylibThread) -> Self {
        Self { raylib_thread }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadExpect<'a, Application>,
        WriteExpect<'a, RaylibContext>,
        ReadStorage<'a, Rendering>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (application, mut raylib_context, renderings, positions): Self::SystemData) {
        let draw = raylib_context.handle.begin_drawing(&self.raylib_thread);
        let mut renderer = RaylibRenderer::create(draw);

        renderer.clear_background(&application.clear_color);
        for (rendering, position) in (&renderings, &positions).join() {
            renderer.render(rendering, position);
        }
        
        // gfx.clear_background(0xfff);


        // if self.raylib.handle.window_should_close() {
        //     application.window_should_close = true;
        //     return;
        // }

        // let mut d = self.raylib.handle.begin_drawing(&self.raylib.thread);
        // d.clear_background(Color::WHITE);

        // for (position, circle) in (&positions, &circles).join() {
        //     d.draw_circle(position.vector.x as i32, position.vector.y as i32, circle.radius, raylib::core::color::Color::RED);
        // }

        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}