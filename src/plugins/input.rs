use bevy::prelude::*;
use bevy_asset_loader::loading_state::LoadingStateSet;

use crate::{
    labels::{GamePlaySet, GameState},
    systems::player,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        // Always collect input from player before all other systems
        app.add_systems(
            Update,
            (player::get_player_ship_input)
                .in_set(GamePlaySet::Input)
                .after(LoadingStateSet(GameState::AssetLoading)), // appease the system ordering gods
        );
    }
}
