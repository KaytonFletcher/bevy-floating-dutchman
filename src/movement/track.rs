use bevy::prelude::*;

use crate::components::Track;

/**
 * Runs when the Track component is modified. Currently this just changes the entities transform to point
 * the entity marked on the Track component using Trigonometry. In the future we may want to use &mut ExternalForce
 * to apply a more "weighty" rotational feel rather than snapping to where the entity should look.
 * Move this to "Physics" GamePlay set if we decide to use forces here.
 */
pub fn update_track_direction(mut trackers: Query<(&Track, &mut Transform)>) {
    for (track, mut transform) in trackers.iter_mut() {
        // angle between entity (rigid body) being tracked and the entity given the Track component
        let new_angle = (track.pos.y - transform.translation.y)
            .atan2(track.pos.x - transform.translation.x)
            + track.get_offset();

        transform.rotation = Quat::from_rotation_z(new_angle);
    }
}

pub fn update_tracked_positions(
    mut trackers: Query<&mut Track>,
    tracked_query: Query<&Transform, Changed<Transform>>,
) {
    for mut track in trackers.iter_mut() {
        if let Some(tracked_entity) = &track.entity_tracked {
            if let Ok(transform) = tracked_query.get(*tracked_entity) {
                track.pos.x = transform.translation.x;
                track.pos.y = transform.translation.y;
            }
        }
    }
}
