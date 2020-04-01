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

fn update_screen_coordinates(screen_coordinates: &mut ScreenCoordinates, 
    application: &Application, 
    position: &Position, 
    coordinate_system: &CoordinateSystem)
{
    // according the the position's coordinate system, we will know how to update
    match coordinate_system {
        CoordinateSystem::Screen => {
            screen_coordinates.x = position.vector.x as i32;
            screen_coordinates.y = position.vector.y as i32;
        },
        CoordinateSystem::Ndc => {
            screen_coordinates.x = (position.vector.x * application.window.resolution.width as f32) as i32;
            screen_coordinates.y = (position.vector.y * application.window.resolution.height as f32) as i32;
        }
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

        // we keep a single value and update it for each render target
        let mut screen_coordinates = ScreenCoordinates::new(0, 0);

        renderer.clear_background(&application.window.color);
        for (rendering, position) in (&renderings, &positions).join() {
            update_screen_coordinates(&mut screen_coordinates, 
                &application, position,
                rendering.coordinate_system.as_ref().unwrap_or(&CoordinateSystem::Screen));
            renderer.render(rendering, &screen_coordinates);
        }
    }
}