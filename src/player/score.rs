use bevy::prelude::*;

use crate::{components::Player, labels::events::PlayerScored};

pub fn add_scores_from_killed(
    mut player_scored_events: EventReader<PlayerScored>,
    mut player_query: Query<&mut Player>,
) {
    for PlayerScored { player, score } in player_scored_events.read() {
        if let Result::Ok(mut player) = player_query.get_mut(*player) {
            player.score += score;
            println!("Player score is now: {}", player.score)
        }
    }
}
