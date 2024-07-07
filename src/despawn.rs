use bevy::prelude::*;

use crate::{
    components::Health,
    labels::{events::EntityKilled, sets::GamePlaySet, states::GameState},
};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_dead_entities.in_set(GamePlaySet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_dead_entities(mut commands: Commands, mut entities_killed: EventReader<EntityKilled>) {
    for EntityKilled(e1, _) in entities_killed.read() {
        info!("Despawning dead enemy {:?}", e1);
        // Some entities may be destroyed by other means
        // Ex. This could have a race condition with Projectile TTL
        commands.entity(*e1).despawn_recursive();
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
