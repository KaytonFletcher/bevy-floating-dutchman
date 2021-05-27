use bevy::{
    ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*, render::render_graph::Stages,
};

use bevy_rapier2d::physics::RapierPhysicsPlugin;

use bevy_asset_loader::AssetLoader;

use events::WeaponFired;
use labels::{CustomStages, GameState};
use resources::SpriteAssets;

mod components;
mod entities;
mod events;
mod labels;
mod resources;
mod systems;
mod ui;

fn main() {
    let mut app = App::build();

    AssetLoader::new(GameState::AssetLoading, GameState::InitialSpawn)
        .with_collection::<SpriteAssets>()
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "The Floating Dutchman".to_string(),
        width: 1920.0,
        height: 1080.0,
        ..Default::default()
    })
    .add_event::<WeaponFired>()
    .add_plugin(RapierPhysicsPlugin)
    .add_plugins(DefaultPlugins)
    .add_stage_after(
        CoreStage::Update,
        CustomStages::Physics,
        SystemStage::single_threaded(),
    )
    .add_state(GameState::AssetLoading)
    .add_state_to_stage(CustomStages::Physics, GameState::AssetLoading)
    // .add_stage_after(
    //     CustomStages::Physics,
    //     CustomStages::Debug,
    //     SystemStage::single_threaded(),
    // )
    .add_system_set(
        SystemSet::on_enter(GameState::InitialSpawn)
            .with_system(systems::setup.system().label("setup"))
            .with_system(
                entities::spawn_player
                    .system()
                    .label("player")
                    .after("setup"),
            )
            .with_system(entities::spawn_follow_enemy.system().after("setup"))
            .with_system(ui::spawn_player_ui.system().after("player")),
    )
    .add_system_set(
        SystemSet::on_enter(GameState::Playing)
            
            // .with_system(entities::spawn_shoot_enemy.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .with_system(systems::weapon_fire_rate.system().label("weapon_tick"))
            .with_system(
                systems::player_input
                    .system()
                    .label("player_input")
                    // .after("weapon_tick"),
            )
            .with_system(
                systems::constant_weapon_fire
                    .system()
                    .label("constant_weapon_fire")
                    .after("weapon_tick"),
            )
            .with_system(systems::follow.system().label("follow"))
            .with_system(
                systems::tracking
                    .system()
                    .label("tracking")
                    .after("player_input")
                    .after("follow"),
            )
            .with_system(systems::position_system.system())
            .with_system(systems::despawn_projectile.system())
            .with_system(ui::update_player_ui.system())
            .with_system(
                systems::weapon_fired
                    .system()
                    .after("player_input")
                    .after("constant_weapon_fire"),
            ),
    )
    .add_system_set_to_stage(
        CustomStages::Physics,
        SystemSet::on_update(GameState::Playing)
            .with_system(systems::update_movement.system())
            .with_system(systems::update_tracking.system())
            .with_system(systems::collision.system()),
    )
    .run();
    // .add_system_to_stage(
    //     CustomStages::Debug,
    //     systems::debug::debug_projectiles.system(),
    // )
    // .insert_resource(ReportExecutionOrderAmbiguities)
    // .add_plugin(RapierRenderPlugin)
}
