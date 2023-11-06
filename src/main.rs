use bevy::{prelude::*, window::WindowResolution};

use bevy_rapier2d::{
    plugin::RapierPhysicsPlugin, prelude::NoUserData, render::RapierDebugRenderPlugin,
};

use events::{WeaponFired, PlayerKilled, EntityKilled};
use labels::{GamePlaySet, GameState, MainSet};
use plugins::AssetLoadingPlugin;

mod components;
mod entities;
mod events;
mod labels;
mod plugins;
mod resources;
mod systems;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Option::Some(Window {
            title: "The floating dutchman".to_owned(),
            resolution: WindowResolution::new(1920.0, 1080.0),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_state::<GameState>()
    .add_plugins(AssetLoadingPlugin)
    .add_event::<WeaponFired>()
    .add_event::<EntityKilled>()
    .add_event::<PlayerKilled>()
    .configure_sets(
        Update,
        (MainSet::GamePlay.run_if(in_state(GameState::Playing)),),
    )
    .configure_sets(
        PostUpdate,
        (MainSet::GamePlay.run_if(in_state(GameState::Playing)),),
    )
    .configure_sets(
        Last,
        (MainSet::GamePlay.run_if(in_state(GameState::Playing)),),
    )
    // one-time systems for setting up the world space
    // may be able to add these to startup schedule instead
    .add_systems(
        OnEnter(GameState::SpawnPlayer),
        (systems::setup, entities::spawn_player, ui::spawn_player_ui).chain(),
    )
    .add_systems(
        OnEnter(GameState::SpawnEnemies),
        (entities::spawn_shoot_enemy, entities::spawn_follow_enemy).chain(),
    )
    // should run player input before all other update systems
    .add_systems(
        Update,
        (
            (systems::player_input).in_set(GamePlaySet::Input),
            // We run weapon systems in a strict order here
            (
                systems::weapon_fire_rate,
                systems::constant_weapon_fire,
                systems::weapon_fired,
            )
                .chain()
                .after(GamePlaySet::Input),
            // systems we are okay with running in parallel during main loop
            (
                systems::tracking,
                systems::follow,
                systems::boundary_position_system,
                systems::despawn_projectile,
                systems::handle_death,
                ui::update_player_ui,
            )
                .after(GamePlaySet::Input),
        )
            .in_set(MainSet::GamePlay),
    )
    // physiscs sytems run in parallel after "simulation" steps in CoreSet::PostUpdate
    .add_systems(
        PostUpdate,
        (systems::update_movement, systems::update_tracking).in_set(MainSet::GamePlay),
    )
    // apply collision detection last, after all translations to entities have completed
    .add_systems(Last, systems::collision.in_set(MainSet::GamePlay))
    .run();
    // .add_system_to_stage(
    //     CustomStages::Debug,
    //     systems::debug::debug_projectiles,
    // )
}
