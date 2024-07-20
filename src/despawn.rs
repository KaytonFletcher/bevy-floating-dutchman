use bevy::prelude::*;

use crate::{
    components::Health,
    labels::{sets::GamePlaySet, states::GameState},
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

fn despawn_dead_entities(mut commands: Commands, health_query: Query<(Entity, &Health)>) {
    for (entity, Health { current_health, .. }) in health_query.iter() {
        if *current_health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
