use bevy::prelude::*;

use bevy_rapier2d::physics::RapierConfiguration;

pub struct MainCamera;

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.scale = 20.0;
    rapier_config.gravity = Vec2::ZERO.into();
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
