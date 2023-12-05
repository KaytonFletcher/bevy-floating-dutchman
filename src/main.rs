use bevy::prelude::App;

use plugins::{AssetLoadingPlugin, MainShipGameplayPlugin, SetupPlugin};

mod components;
mod entities;
mod events;
mod labels;
mod plugins;
mod resources;
mod systems;

fn main() {
    let mut app = App::new();

    app
        // First we load assets
        .add_plugins(AssetLoadingPlugin)
        // Then we set up world
        .add_plugins(SetupPlugin)
        // Then we run game
        .add_plugins(MainShipGameplayPlugin)
        .run();
}
