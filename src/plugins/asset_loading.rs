use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    labels::GameState,
    resources::{SpriteAssets, UIAssets},
};

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::SpawnPlayer),
        )
        .add_collection_to_loading_state::<_, SpriteAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, UIAssets>(GameState::AssetLoading);
    }
}
