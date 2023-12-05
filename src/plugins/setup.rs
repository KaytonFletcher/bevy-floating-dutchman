use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    window::WindowResolution,
};

use bevy_rapier2d::{
    plugin::RapierPhysicsPlugin, prelude::NoUserData, render::RapierDebugRenderPlugin,
};

use crate::{
    entities,
    events::{EntityKilled, PlayerKilled, WeaponFired},
    labels::GameState,
    systems::{setup, ui},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // Configure main window
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::Some(Window {
                title: "The floating dutchman".to_owned(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                ..default()
            }),
            ..default()
        }))
        // Enable system ambiguity detection
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        // setup rapier physics (used for forces, collision, etc...)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // register states and events here
        .add_state::<GameState>()
        .add_event::<WeaponFired>()
        .add_event::<EntityKilled>()
        .add_event::<PlayerKilled>()
        // one-time systems for setting up the world space
        // may be able to add these to startup schedule instead
        .add_systems(
            OnEnter(GameState::SpawnPlayer),
            (
                setup::setup_basic_config,
                entities::spawn_player,
                ui::spawn_player_ui,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::SpawnEnemies),
            (entities::spawn_shoot_enemy, entities::spawn_follow_enemy).chain(),
        );
    }
}