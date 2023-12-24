use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::{
    components::{Damage, Health, Player, Projectile},
    labels::events::{EntityKilled, PlayerKilled},
};

pub fn handle_collisions(
    mut damaged_query: Query<&mut Health>,
    player_query: Query<Entity, With<Player>>,
    damager_query: Query<&Damage>,
    mut projectile_query: Query<&mut Projectile>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
    mut entity_killed_event_writer: EventWriter<EntityKilled>,
    mut player_killed_event_writer: EventWriter<PlayerKilled>,
) {
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                info!("Collision Event Started: {:?}, {:?}", e1, e2);
                // If e1 has health and e2 deals damage, apply e2 damage to e1 health
                if dealt_damage_and_is_dead(
                    &mut damaged_query,
                    &damager_query,
                    e1,
                    e2,
                    &mut projectile_query,
                ) {
                    publish_entity_killed(
                        &mut entity_killed_event_writer,
                        &mut player_killed_event_writer,
                        *e1,
                        *e2,
                        player_query.contains(*e1),
                    )
                }

                // Reverses, now e1 is dealing damage to e2
                if dealt_damage_and_is_dead(
                    &mut damaged_query,
                    &damager_query,
                    e2,
                    e1,
                    &mut projectile_query,
                ) {
                    publish_entity_killed(
                        &mut entity_killed_event_writer,
                        &mut player_killed_event_writer,
                        *e2,
                        *e1,
                        player_query.contains(*e2),
                    )
                }
            }

            _ => (),
        }
    }

    for _contact_event in contact_event_reader.read() {}
}

fn dealt_damage_and_is_dead(
    damaged_query: &mut Query<&mut Health>,
    damager_query: &Query<&Damage>,
    e1: &Entity,
    e2: &Entity,
    projectile_query: &mut Query<&mut Projectile>,
) -> bool {
    return if let (Ok(mut health), Ok(damage)) =
        (damaged_query.get_mut(*e1), damager_query.get(*e2))
    {
        // If the damager is a projectile, only want to damage if it has "hits" left
        if let Ok(mut projectile) = projectile_query.get_mut(*e2) {
            if projectile.add_hit() {
                health.damage(damage.amount)
            } else {
                false
            }
        } else {
            health.damage(damage.amount)
        }
    } else {
        false
    };
}

fn publish_entity_killed(
    entity_killed_event_writer: &mut EventWriter<EntityKilled>,
    player_killed_event_writer: &mut EventWriter<PlayerKilled>,
    entity_killed: Entity,
    entity_killing: Entity,
    is_player: bool,
) {
    if is_player {
        player_killed_event_writer.send(PlayerKilled(entity_killed))
    } else {
        entity_killed_event_writer.send(EntityKilled(entity_killed, entity_killing))
    }
}
