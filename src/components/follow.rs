use bevy::prelude::Entity;

pub struct Follow {
    pub entity: Entity,
}

impl Follow {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
