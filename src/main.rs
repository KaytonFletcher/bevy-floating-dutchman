use bevy::{ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*};

use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod components;
mod entities;
mod resources;
mod systems;

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
        .add_system(systems::player_movement.system().label("player_movement"))
        .add_system(
            systems::enemy_tracking
                .system()
                .label("enemy_tracking")
                .after("player_movement"),
        )
        .add_system(
            systems::update_movement
                .system()
                .label("movement")
                .after("player_movement")
                .after("enemy_tracking"),
        )
        .add_system(systems::update_tracking.system().after("movement"))
        .add_system(
            systems::physics_dampening
                .system()
                .label("dampening")
                .after("movement"),
        )
        .add_system(systems::position_system.system().after("dampening"))
        .insert_resource(ReportExecutionOrderAmbiguities)
        // .add_plugin(RapierRenderPlugin)
        .run();
}
