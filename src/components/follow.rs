use bevy::prelude::Entity;

pub struct Follow {
    pub entity: Entity,
    pub space: Option<f32>,
}

impl Follow {
    pub fn new(entity: Entity) -> Self {
        Self { entity, space: None }
    }
}
