extern crate log;
extern crate log4rs;
extern crate raylib;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod components;
pub mod game;
pub mod prelude;
pub mod resources;
pub mod services;
pub mod states;
pub mod systems;
pub mod util;

fn main() {
    let mut game = game::Game::initialize(util::load_yaml_resource("assets/application.yaml"));

    while !game.should_close() {
        game.update();
    }
}
