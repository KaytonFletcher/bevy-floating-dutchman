use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    window::WindowResolution,
};

use bevy_rapier2d::prelude::*;

use crate::{
    entities,
    labels::{
        events::{EntityKilled, PlayerKilled, WeaponFired},
        states::GameState,
        CursorCoordinates, MainCamera,
    },
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
        .init_state::<GameState>()
        .add_event::<WeaponFired>()
        .add_event::<EntityKilled>()
        .add_event::<PlayerKilled>()
        // one-time systems for setting up the world space
        // may be able to add these to startup schedule instead
        .add_systems(
            OnEnter(GameState::AssetLoading),
            (setup_camera, setup_rapier_physics, setup_cursor).chain(),
        )
        .add_systems(
            OnEnter(GameState::SpawnEnemies),
            (entities::spawn_shoot_enemy, entities::spawn_follow_enemy).chain(),
        );
    }
}

fn setup_camera(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO.into();

    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn setup_rapier_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Disable gravity
    rapier_config.gravity = Vec2::ZERO.into();
}

fn setup_cursor(mut commands: Commands) {
    commands
        .spawn(CursorCoordinates)
        .insert(Transform::from_translation(Vec3::ZERO));
}
