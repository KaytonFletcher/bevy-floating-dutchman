use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::{
    components::{Damage, Health},
    labels::events::DealDamage,
};

pub fn handle_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
    damaged_query: Query<&Health>,
    damager_query: Query<&Damage>,
) {
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                info!("Collision Event Started: {:?}, {:?}", e1, e2);

                // Perform collision from e1 -> e2 and e2 -> e1 so both have the others damage applied
                for (e1, e2) in [(*e1, *e2), (*e2, *e1)] {
                    // If e1 has health and e2 deals damage, apply e2 damage to e1 health
                    if let (Ok(_health), Ok(damage)) =
                        (damaged_query.get(e1), damager_query.get(e2))
                    {
                        commands.trigger_targets(
                            DealDamage {
                                damage: damage.amount,
                                cause: e2,
                            },
                            e1,
                        );
                    }
                }
            }

            _ => (),
        }
    }

    for _contact_event in contact_event_reader.read() {}
}
