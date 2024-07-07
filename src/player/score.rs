use bevy::prelude::*;

use crate::{
    components::{Player, Score, Scorer},
    labels::events::{EntityKilled, PlayerScored},
};

pub fn publish_scores_from_killed(
    mut entities_killed: EventReader<EntityKilled>,
    mut player_scored_event_writer: EventWriter<PlayerScored>,
    score_query: Query<&Score>,
    scorer_query: Query<&Scorer>,
) {
    for EntityKilled(e1, e2) in entities_killed.read() {
        info!("Trying to publish score for entity killed {:?}", e1);

        // Does the entity have a score? Emit an event for the player to receive the score
        if let Result::Ok(Score { score }) = score_query.get(*e1) {
            if let Result::Ok(Scorer { player }) = scorer_query.get(*e2) {
                player_scored_event_writer.send(PlayerScored {
                    score: *score,
                    player: *player,
                });
            }
        }
    }
}

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
