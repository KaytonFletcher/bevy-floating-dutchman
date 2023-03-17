use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

#[derive(Component)]
pub struct MainCamera;

pub fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO.into();
    // rapier_config.time_dependent_number_of_timesteps = true;

    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}
