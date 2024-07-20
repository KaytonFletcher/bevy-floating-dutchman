use bevy::prelude::*;

use crate::labels::sets::{GamePlaySet, MovementSet};

mod follow;
mod force;
mod track;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // Track systems run in a strict order
            (
                track::update_tracked_positions,
                track::update_track_direction,
                follow::update_follow_direction,
            )
                .chain()
                .in_set(MovementSet)
                .in_set(GamePlaySet::Simulation),
        )
        .add_systems(
            Update,
            force::apply_forces_from_acceleration.in_set(GamePlaySet::Physics),
        );
    }
}
