use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::{
    components::{Damage, Health, Player},
    events::{EntityKilled, PlayerKilled},
};

pub fn collision(
    mut damaged_query: Query<&mut Health>,
    mut player_query: Query<Entity, With<Player>>,
    damager_query: Query<&Damage>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
    mut entity_killed_event_writer: EventWriter<EntityKilled>,
    mut player_killed_event_writer: EventWriter<PlayerKilled>,
) {
    let player = player_query.get_single_mut().unwrap();

    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                // If e1 has health and e2 deals damage, apply e2 damage to e1 health
                if dealt_damage_and_is_dead(&mut damaged_query, &damager_query, e1, e2) {
                    publish_entity_killed(
                        &mut entity_killed_event_writer,
                        &mut player_killed_event_writer,
                        *e1,
                        player,
                    )
                }

                // Reverses, now e1 is dealing damage to e2
                if dealt_damage_and_is_dead(&mut damaged_query, &damager_query, e2, e1) {
                    publish_entity_killed(
                        &mut entity_killed_event_writer,
                        &mut player_killed_event_writer,
                        *e2,
                        player,
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
) -> bool {
    return if let (Ok(mut health), Ok(damage)) =
        (damaged_query.get_mut(*e1), damager_query.get(*e2))
    {
        health.damage(damage.amount)
    } else {
        false
    };
}

fn publish_entity_killed(
    entity_killed_event_writer: &mut EventWriter<EntityKilled>,
    player_killed_event_writer: &mut EventWriter<PlayerKilled>,
    entity_killed: Entity,
    player: Entity,
) {
    if entity_killed == player {
        player_killed_event_writer.send(PlayerKilled(player))
    } else {
        entity_killed_event_writer.send(EntityKilled(entity_killed))
    }
}
