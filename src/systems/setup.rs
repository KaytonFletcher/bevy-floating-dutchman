use bevy::prelude::*;

use bevy_rapier2d::physics::RapierConfiguration;

use crate::ui::spawn_player_ui;

pub struct MainCamera;

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    rapier_config.scale = 20.0;
    rapier_config.gravity = Vec2::ZERO.into();
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    spawn_player_ui(commands, materials, asset_server)
}
