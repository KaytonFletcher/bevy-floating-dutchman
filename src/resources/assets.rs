use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::labels::states::GameState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::SpawnPlayer)
                .load_collection::<SpriteAssets>()
                .load_collection::<UIAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/green_fighter.png")]
    pub follow_enemy: Handle<Image>,
    #[asset(path = "sprites/pirate_ship.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/cannonball.png")]
    pub cannonball: Handle<Image>,
    #[asset(path = "sprites/red_fighter.png")]
    pub shoot_enemy: Handle<Image>,
    #[asset(path = "sprites/laser_shot_1.png")]
    pub enemy_bullet: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "sprites/heart_outline.png")]
    pub heart_outline: Handle<Image>,
    #[asset(path = "sprites/half_heart.png")]
    pub heart_half: Handle<Image>,
    #[asset(path = "sprites/full_heart.png")]
    pub heart_full: Handle<Image>,
}
