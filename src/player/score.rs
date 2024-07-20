use bevy::prelude::*;

use crate::{
    components::{Player, Score, Scorer},
    labels::events::{EntityDeath, PlayerScored},
};

pub fn publish_scores_from_killed(
    trigger: Trigger<EntityDeath>,
    mut commmands: Commands,
    score_query: Query<&Score>,
    scorer_query: Query<&Scorer>,
) {
    info!(
        "Trying to publish score for entity killed {:?}",
        trigger.entity()
    );

    // Does the entity have a score? Emit an event for the player to receive the score
    if let Result::Ok(Score { score }) = score_query.get(trigger.entity()) {
        if let Result::Ok(Scorer { player }) = scorer_query.get(trigger.event().cause) {
            info!("Score published for entity killed {:?}", trigger.entity());
            commmands.trigger_targets(PlayerScored { score: *score }, *player);
        }
    }
}

pub fn add_scores_from_killed(
    trigger: Trigger<PlayerScored>,
    mut player_query: Query<&mut Player>,
) {
    if let Result::Ok(mut player) = player_query.get_mut(trigger.entity()) {
        player.score += trigger.event().score;
        println!("Player score is now: {}", player.score)
    }
}
