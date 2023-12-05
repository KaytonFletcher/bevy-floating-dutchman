use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct Follow {
    pub entity: Entity,
    pub space: Option<f32>,
}

impl Follow {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            space: None,
        }
    }

    pub fn get_new_angle_to_follow(
        &self,
        transform: &Transform,
        followed_transform: &Transform,
    ) -> Quat {
        let pos = transform.translation;
        let being_followed_pos = followed_transform.translation;

        // we calculate the x and y translation between the entity being followed
        // and the entity following. This ensures the follower will move in the direction
        // of the entity (Rigid body component) being followed
        let new_follow_pos = (being_followed_pos - pos).truncate();

        // angle between entity (rigid body) being tracked and the entity given the Track component
        let mut new_angle = (new_follow_pos.y).atan2(new_follow_pos.x);

        if let Some(space) = self.space {
            if new_follow_pos.length() < space {
                new_angle = new_angle + PI;
            }
        }

        Quat::from_rotation_z(new_angle)
    }
}
