use bevy::prelude::*;

use crate::{
    components::{Player, Score, Scorer},
    events::EntityKilled,
};

pub fn add_scores_from_killed(
    mut player_query: Query<&mut Player>,
    score_query: Query<&Score>,
    scorer_query: Query<&Scorer>,
    mut entities_killed: EventReader<EntityKilled>,
) {
    for EntityKilled(e1, e2) in entities_killed.read() {
        if let Result::Ok(score) = score_query.get(*e1) {
            println!("Entity killed has a score: {}", score.score);

            if let Result::Ok(scorer) = scorer_query.get(*e2) {
                if let Result::Ok(mut player) = player_query.get_mut(scorer.player) {
                    player.score += score.score;
                    println!("Player score is now: {}", player.score)
                }
            }
        }
    }
}
