use bevy::prelude::*;

use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::components::{Follow, Motion, Player, Track};

pub fn tracking(
    mut trackers: Query<&mut Track, Without<Player>>,
    rb_query: Query<&RigidBodyHandleComponent>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for mut track in trackers.iter_mut() {
        if let Some(entity) = &track.entity_tracked {
            if let Ok(rb_comp) = rb_query.get_component::<RigidBodyHandleComponent>(*entity) {
                if let Some(rb) = rigid_bodies.get(rb_comp.handle()) {
                    track.pos.x = rb.position().translation.x;
                    track.pos.y = rb.position().translation.y;
                }
            }
        }
    }
}

pub fn follow(
    mut followers: Query<(&Follow, &RigidBodyHandleComponent, &mut Motion)>,
    rb_query: Query<&RigidBodyHandleComponent>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for (follow, rb_handle, mut motion) in followers.iter_mut() {
        if let Some(follower_rb) = rigid_bodies.get(rb_handle.handle()) {
            if let Ok(rb_comp) = rb_query.get_component::<RigidBodyHandleComponent>(follow.entity) {
                if let Some(followed_rb) = rigid_bodies.get(rb_comp.handle()) {
                    let follower_pos = follower_rb.position().translation;
                    let followed_pos = followed_rb.position().translation;

                    let x = followed_pos.x - follower_pos.x;
                    let y = followed_pos.y - follower_pos.y;
                    motion.direction = Vec2::new(x, y).normalize();
                    // motion.acceleration = dir;
                }
            }
        }
    }
}
