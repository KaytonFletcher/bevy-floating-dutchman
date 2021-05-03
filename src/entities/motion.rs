use bevy::prelude::Vec2;
pub struct Motion {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_accel: f32,
    pub max_vel: f32,
}

impl Motion {
    pub fn new(max_vel: f32, acceleration: f32) -> Self {
        Motion {
            velocity: Vec2::new(0., 0.),
            acceleration: Vec2::new(0., 0.),
            max_accel: acceleration,
            max_vel,
        }
    }
}
