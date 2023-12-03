use bevy::prelude::*;

use crate::events::{EntityKilled, PlayerKilled};

pub fn handle_death(
    mut commands: Commands,
    mut entities_killed: EventReader<EntityKilled>,
    mut players_killed: EventReader<PlayerKilled>,
    entities: Query<Entity>,
) {
    for EntityKilled(e1, _) in entities_killed.read() {
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
