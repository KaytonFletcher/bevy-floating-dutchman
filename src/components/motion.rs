use bevy::prelude::{Component, Quat};

#[derive(Clone, Component)]
pub struct Motion {
    pub direction: Quat,
    pub is_moving: bool,
    pub acceleration: f32,
    pub max_vel: f32,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            direction: Quat::IDENTITY,
            is_moving: true,
            acceleration: 200.0,
            max_vel: 20.0,
        }
    }
}
