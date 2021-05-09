use bevy::{ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*};

use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod components;
mod entities;
mod resources;
mod systems;
mod ui;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Floating Dutchman".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .insert_resource(resources::Game { player: None })
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup.system().label("setup"))
        .add_startup_system(
            entities::spawn_player
                .system()
                .label("player")
                .after("setup"),
        )
        .add_startup_system(
            entities::spawn_follow_enemy
                .system()
                .label("enemy")
                .after("player"),
        )
        .add_system(systems::player_input.system().label("player_input"))
        .add_system(systems::follow.system().label("follow"))
        .add_system(
            systems::tracking
                .system()
                .label("tracking")
                .after("player_input")
                .after("follow"),
        )
        .add_system(
            systems::update_movement
                .system()
                .label("movement")
                .after("player_input")
                .after("enemy_tracking"),
        )
        .add_system(systems::update_tracking.system().after("movement"))
        .add_system(systems::position_system.system().after("movement"))
        .add_system(systems::collision.system())
        .insert_resource(ReportExecutionOrderAmbiguities)
        // .add_plugin(RapierRenderPlugin)
        .run();
}
