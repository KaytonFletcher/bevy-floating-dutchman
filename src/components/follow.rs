use bevy::prelude::{Entity, Component};

#[derive(Component)]
pub struct Follow {
    pub entity: Entity,
    pub space: Option<f32>,
}

impl Follow {
    pub fn new(entity: Entity) -> Self {
        Self { entity, space: None }
    }
}
