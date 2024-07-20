use bevy::prelude::*;

use crate::labels::{sets::GamePlaySet, states::GameState};

mod create;
mod score;
mod ship;
mod ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SpawnPlayer),
            (create::spawn_player, ui::spawn_player_ui).chain(),
        )
        // Always collect input from player before all other systems
        .add_systems(
            Update,
            (ship::update_cursor_position, ship::get_player_ship_input)
                .in_set(GamePlaySet::PlayerInput),
        )
        .add_systems(
            Update,
            (ui::update_player_health_ui, ui::update_player_score_ui)
                .in_set(GamePlaySet::Simulation),
        )
        .observe(score::publish_scores_from_killed);
    }
}
