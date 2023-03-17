use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::components::{Damage, Health};

pub fn collision(
    mut commands: Commands,
    mut player_query: Query<&mut Health>,
    damage_query: Query<&Damage>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_event_reader.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, flag) => {
                if let Ok(mut health) = player_query.get_component_mut::<Health>(*e1) {
                    if let Ok(damage) = damage_query.get_component::<Damage>(*e2) {
                        if health.damage(damage.amount) {
                            commands.entity(*e1).despawn();
                        }
                    }
                }

                if let Ok(mut health) = player_query.get_component_mut::<Health>(*e2) {
                    if let Ok(damage) = damage_query.get_component::<Damage>(*e1) {
                        if health.damage(damage.amount) {
                            commands.entity(*e2).despawn();
                        }
                    }
                }
            }
            _ => (),
        }
    }

    for contact_event in contact_event_reader.iter() {}
}
