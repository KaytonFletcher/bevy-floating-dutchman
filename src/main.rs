use bevy::prelude::*;

mod components;
mod entities;
mod resources;
mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup.system())
        .add_startup_system(entities::init_player.system())
        .add_system(systems::animate_player.system())
        .run();
}
