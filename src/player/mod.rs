use bevy::prelude::*;

use crate::{
    entities,
    labels::{sets::GamePlaySet, states::GameState},
};

mod score;
mod ship;
mod ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SpawnPlayer),
            (entities::spawn_player, ui::spawn_player_ui).chain(),
        )
        // Always collect input from player before all other systems
        .add_systems(
            Update,
            (ship::update_cursor_position, ship::get_player_ship_input)
                .in_set(GamePlaySet::PlayerInput),
        )
        .add_systems(
            Update,
            (
                ship::all_players_destroyed,
                score::add_scores_from_killed,
                ui::update_player_health_ui,
                ui::update_player_score_ui,
            )
                .in_set(GamePlaySet::Simulation),
        )
        .add_systems(
            Update,
            score::publish_scores_from_killed.in_set(GamePlaySet::Cleanup),
        );
    }
}
