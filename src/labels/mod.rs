use bevy::prelude::Component;

pub mod events;
pub mod sets;
pub mod states;

/**
 * Here we include all "marker" components that have no associated data.
 */
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct CursorCoordinates;
