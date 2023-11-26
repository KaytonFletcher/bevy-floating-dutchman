use bevy::prelude::*;

use crate::{
    labels::{GamePlaySet, MainSet},
    systems,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // physiscs sytems run in parallel after "simulation" steps in CoreSet::PostUpdate
        app.add_systems(
            Update,
            (systems::update_movement)
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Physics)
                .after(GamePlaySet::Simulation),
        );
    }
}
