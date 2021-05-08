use bevy::prelude::Vec2;
pub struct Motion {
    pub direction: Vec2,
    pub force: f32,
    pub max_vel: f32,
}

impl Motion {
    pub fn new(max_vel: f32, force: f32) -> Motion {
        Self {
            direction: Vec2::new(0., 0.),
            force,
            max_vel,
        }
    }
}
