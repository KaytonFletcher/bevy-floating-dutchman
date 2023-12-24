use bevy::prelude::*;

use crate::labels::sets::GamePlaySet;

mod boundary;
mod collision_detection;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // apply collision detection last, after all translations to entities have completed
        app.add_systems(
            Update,
            (collision_detection::handle_collisions).in_set(GamePlaySet::Collision),
        )
        .add_systems(
            Update,
            boundary::wrap_around_screen_edge.in_set(GamePlaySet::Simulation),
        );
    }
}
