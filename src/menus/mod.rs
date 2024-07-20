use bevy::prelude::*;

use crate::labels::states::GameState;

mod pause;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), pause::create_pause_menu)
            .add_systems(OnEnter(GameState::Paused), pause::on_pause)
            .add_systems(OnExit(GameState::Paused), pause::on_resume_game)
            .add_systems(
                Update,
                pause::return_to_game.run_if(in_state(GameState::Paused)),
            );
    }
}
