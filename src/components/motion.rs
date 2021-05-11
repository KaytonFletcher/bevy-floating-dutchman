use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct Motion {
    pub direction: Vec2,
    pub acceleration: f32,
    pub max_vel: f32,
}

impl Motion {
    pub fn new(max_vel: f32, acceleration: f32) -> Motion {
        Self {
            direction: Vec2::new(0., 0.),
            acceleration,
            max_vel,
        }
    }
}
