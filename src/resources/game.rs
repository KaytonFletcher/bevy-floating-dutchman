use bevy::prelude::*;

use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(path = "sprites/green_fighter.png")]
    pub follow_enemy: Handle<Texture>,
    #[asset(path = "sprites/pirate_ship.png")]
    pub player: Handle<Texture>,
    #[asset(path = "sprites/cannonball.png")]
    pub cannonball: Handle<Texture>,
    #[asset(path = "sprites/red_fighter.png")]
    pub shoot_enemy: Handle<Texture>,
    #[asset(path = "sprites/laser_shot_1.png")]
    pub enemy_bullet: Handle<Texture>,
}
