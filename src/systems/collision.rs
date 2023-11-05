use std::ops::Not;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::components::{Damage, Health, Player};

pub fn collision(
    mut commands: Commands,
    mut damaged_query: Query<&mut Health>,
    mut player_query: Query<(Entity, &mut Player)>,
    damager_query: Query<&Damage>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
) {
    let (player_entity, mut player) = player_query.get_single_mut().unwrap();

    for collision_event in collision_event_reader.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                // If e1 has health and e2 deals damage, apply e2 damage to e1 health
                if_valid_deal_damage(&mut damaged_query, &damager_query, e1, e2);

                // Reverses, now e1 is dealing damage to e2
                if_valid_deal_damage(&mut damaged_query, &damager_query, e2, e1);
            }

            _ => (),
        }
    }

    for contact_event in contact_event_reader.iter() {}
}

fn if_valid_deal_damage(
    damaged_query: &mut Query<&mut Health>,
    damager_query: &Query<&Damage>,
    e1: &Entity,
    e2: &Entity,
) {
    if let (Ok(mut health), Ok(damage)) = (damaged_query.get_mut(*e1), damager_query.get(*e2)) {
        if health.damage(damage.amount) {}
    }
}
