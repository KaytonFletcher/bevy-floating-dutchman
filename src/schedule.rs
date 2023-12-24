use bevy::prelude::*;
use bevy_asset_loader::loading_state::LoadingStateSet;

use crate::labels::{
    sets::{GamePlaySet, MainSet},
    states::GameState,
};

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            MainSet::GamePlay.run_if(in_state(GameState::Playing)),
        )
        // Configuring the ordering of our gameplay loop using these main sets:
        // Input -> Simulation -> Physics -> Collision
        .configure_sets(
            Update,
            (
                GamePlaySet::DespawnEntities.after(LoadingStateSet(GameState::AssetLoading)), // appease the system ordering gods
                GamePlaySet::PlayerInput,
                GamePlaySet::Simulation,
                GamePlaySet::Physics,
                GamePlaySet::Collision,
            )
                .chain()
                .in_set(MainSet::GamePlay),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(GamePlaySet::DespawnEntities)
                .before(GamePlaySet::PlayerInput),
        );
    }
}
