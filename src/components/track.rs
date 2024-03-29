use bevy::prelude::{Component, Entity, Vec2};

#[derive(Component)]
pub struct Track {
    pub pos: Vec2,
    pub rotate_acceleration: f32,
    pub entity_tracked: Option<Entity>,
    offset: f32,
}

impl Track {
    pub fn new(rotate_acceleration: f32, offset: f32) -> Track {
        Self {
            pos: Vec2::ZERO,
            entity_tracked: None,
            rotate_acceleration,
            offset,
        }
    }

    pub fn with_entity(mut self, entity: Entity) -> Track {
        self.entity_tracked = Some(entity);
        self
    }

    pub fn get_offset(&self) -> f32 {
        self.offset
    }
}
