extern crate log;
extern crate log4rs;
extern crate raylib;

extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

pub mod util;
pub mod services;
pub mod states;
pub mod components;
pub mod systems;
pub mod resources;
pub mod prelude;
pub mod game;

fn main() {
    
    let mut game = game::Game::initialize(
        util::load_yaml_resource("assets/application.yaml")
    );

    while !game.should_close() {
        game.update();
    }
}