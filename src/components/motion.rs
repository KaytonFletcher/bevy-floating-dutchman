use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct Motion {
    pub direction: Vec2,
    pub acceleration: f32,
    pub max_vel: f32,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            direction: Vec2::new(0.0, 0.0),
            acceleration: 200.0,
            max_vel: 400.0,
        }
    }
}
