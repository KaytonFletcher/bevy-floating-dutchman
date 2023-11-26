use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateSet};

use crate::{
    labels::{GamePlaySet, GameState, MainSet},
    systems,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        // Always collect input from player before all other systems
        app.add_systems(
            Update,
            (systems::player_input)
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Input)
                .after(LoadingStateSet(GameState::AssetLoading)) // appease the system ordering gods
                .before(GamePlaySet::Simulation),
        );
    }
}
