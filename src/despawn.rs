use bevy::prelude::*;

use crate::labels::{
    events::{EntityKilled, PlayerKilled},
    sets::GamePlaySet,
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
    entities: Query<Entity>,
) {
    for EntityKilled(e1, _) in entities_killed.read() {
        info!("Entity Killed {:?}", e1);
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
