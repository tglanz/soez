extern crate log;
extern crate log4rs;
extern crate raylib;

extern crate serde;
extern crate serde_yaml;
extern crate serde_json;

pub mod services;
pub mod states;
pub mod components;
pub mod systems;
pub mod resources;
pub mod prelude;
pub mod game;

fn load_application_settings() -> resources::application::Application {
    let path = "assets/application.yaml";
    let content = std::fs::read_to_string(path)
        .expect(&format!("failed to read file at: {:#}", path));
    serde_yaml::from_str(&content)
        .expect(&format!("failed to deserialize file at: {:#}", path))
}

fn main() {
    
    let mut game = game::Game::initialize(
        load_application_settings()
    );

    while !game.should_close() {
        game.update();
    }
}