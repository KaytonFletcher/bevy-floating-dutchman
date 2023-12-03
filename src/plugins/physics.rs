use bevy::prelude::*;

use crate::{labels::GamePlaySet, systems::movement};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // physiscs sytems run in parallel after "simulation" steps, all in Update core schedule
        app.add_systems(
            Update,
            (movement::apply_forces).in_set(GamePlaySet::Physics),
        );
    }
}
