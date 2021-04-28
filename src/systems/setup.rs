use bevy::prelude::*;
pub struct MainCamera;

pub fn setup(mut commands: Commands) {
    print!("Adding 2d camera\n");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
