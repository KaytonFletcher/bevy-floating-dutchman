use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;

use crate::components::{Follow, Motion};

/**
 * The Follow component today only works in tandem with the Motion component.
 * Here we set the Motion.direction for each
 */
pub fn update_follow_direction(
    mut followers: Query<(&mut Motion, &Follow, &Transform), With<RigidBody>>,
    being_followed: Query<&Transform, With<RigidBody>>,
) {
    for (mut motion, follow, follower_transform) in followers.iter_mut() {
        if let Ok(followed_transform) = being_followed.get(follow.entity) {
            motion.direction =
                follow.get_new_angle_to_follow(follower_transform, followed_transform);
        }
    }
}
