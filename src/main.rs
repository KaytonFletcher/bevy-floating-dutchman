use bevy::prelude::App;
use resources::AssetLoadingPlugin;
use schedule::SchedulePlugin;
use setup::SetupPlugin;

mod collision;
mod components;
mod debug;
mod despawn;
mod entities;
mod labels;
mod movement;
mod player;
mod resources;
mod schedule;
mod setup;
mod weapon;

fn main() {
    let mut app = App::new();

    app
        // First we load assets
        .add_plugins(AssetLoadingPlugin)
        // Set up world: camera, physics settings, etc...
        .add_plugins(SetupPlugin)
        // Configure schedule of game systems
        .add_plugins(SchedulePlugin)
        .add_plugins((
            despawn::DespawnPlugin,
            movement::MovementPlugin,
            player::PlayerPlugin,
            weapon::WeaponPlugin,
            collision::CollisionPlugin,
        ))
        .run();
}
