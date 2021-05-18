use bevy::{
    ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*, render::render_graph::Stages,
};

use bevy_rapier2d::physics::RapierPhysicsPlugin;
use events::WeaponFired;
use labels::CustomStages;

mod components;
mod entities;
mod events;
mod labels;
mod resources;
mod systems;
mod ui;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Floating Dutchman".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .insert_resource(resources::Game { player: None })
        .add_event::<WeaponFired>()
        .add_plugin(RapierPhysicsPlugin)
        .add_plugins(DefaultPlugins)
        .add_stage_after(
            CoreStage::Update,
            CustomStages::Physics,
            SystemStage::single_threaded(),
        )
        .add_stage_after(
            CustomStages::Physics,
            CustomStages::Debug,
            SystemStage::single_threaded(),
        )
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
        .add_startup_system(ui::spawn_player_ui.system().after("player"))
        .add_system(systems::player_input.system().label("player_input"))
        .add_system(systems::follow.system().label("follow"))
        .add_system(
            systems::tracking
                .system()
                .label("tracking")
                .after("player_input")
                .after("follow"),
        )
        .add_system(systems::weapon_fire_rate.system().before("player_input"))
        .add_system(systems::position_system.system())
        .add_system(systems::despawn_projectile.system())
        .add_system(ui::update_player_ui.system())
        .add_system(systems::weapon_fired.system().after("player_input"))
        .add_system_to_stage(CustomStages::Physics, systems::update_movement.system())
        .add_system_to_stage(CustomStages::Physics, systems::update_tracking.system())
        .add_system_to_stage(CustomStages::Physics, systems::collision.system())
        .add_system_to_stage(
            CustomStages::Debug,
            systems::debug::debug_projectiles.system(),
        )
        // .insert_resource(ReportExecutionOrderAmbiguities)
        // .add_plugin(RapierRenderPlugin)
        .run();
}
