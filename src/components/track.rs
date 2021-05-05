use bevy::prelude::Vec2;

pub struct Track {
    pub angle: Vec2,
    pub rotate_speed: f32,
    offset: f32,
}

impl Track {
    pub fn new(rotate_speed: f32, offset: f32) -> Self {
        Self {
            angle: Vec2::ZERO,
            rotate_speed,
            offset,
        }
    }

    pub fn get_offset(&self) -> f32 { self.offset }
}
