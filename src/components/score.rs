use bevy::prelude::Component;

#[derive(Clone, Component)]
pub struct Score {
    pub score: i32,
}
