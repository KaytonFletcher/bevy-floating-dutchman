use bevy::prelude::*;

use crate::labels::{GamePlaySet, GameState, MainSet};

use super::{CollisionPlugin, PhysicsPlugin, PlayerInputPlugin, SimulationPlugin};

pub struct MainShipGameplayPlugin;

impl Plugin for MainShipGameplayPlugin {
    fn build(&self, app: &mut App) {
        // Configuring the ordering of our gameplay loop using these main sets:
        // Input -> Simulation -> Physics -> Collision
        app.configure_sets(
            Update,
            (
                MainSet::GamePlay.run_if(in_state(GameState::Playing)),
                GamePlaySet::Input
                    .in_set(MainSet::GamePlay)
                    .before(GamePlaySet::Simulation),
                GamePlaySet::Simulation
                    .in_set(MainSet::GamePlay)
                    .before(GamePlaySet::Physics),
                GamePlaySet::Physics
                    .in_set(MainSet::GamePlay)
                    .before(GamePlaySet::Collision),
            ),
        )
        .add_plugins((
            PlayerInputPlugin,
            SimulationPlugin,
            PhysicsPlugin,
            CollisionPlugin,
        ));
    }
}
