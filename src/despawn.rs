use bevy::prelude::*;

use crate::{
    components::{Score, Scorer},
    labels::{
        events::{EntityKilled, PlayerKilled, PlayerScored},
        sets::GamePlaySet,
    },
};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_death.in_set(GamePlaySet::DespawnEntities));
    }
}

fn handle_death(
    mut commands: Commands,
    mut entities_killed: EventReader<EntityKilled>,
    mut players_killed: EventReader<PlayerKilled>,
    mut player_scored_event_writer: EventWriter<PlayerScored>,
    entities: Query<Entity>,
    score_query: Query<&Score>,
    scorer_query: Query<&Scorer>,
) {
    for EntityKilled(e1, e2) in entities_killed.read() {
        info!("Entity Killed {:?}", e1);

        // Does the entity have a score? Emit an event for the player to receive the score
        if let Result::Ok(Score { score }) = score_query.get(*e1) {
            if let Result::Ok(Scorer { player }) = scorer_query.get(*e2) {
                player_scored_event_writer.send(PlayerScored {
                    score: *score,
                    player: *player,
                });
            }
        }

        // Some entities may be destroyed by other means
        // Ex. This could have a race condition with Projectile TTL
        if entities.contains(*e1) {
            commands.entity(*e1).despawn();
        }
    }

    for PlayerKilled(player) in players_killed.read() {
        commands.entity(*player).despawn();
    }
}
