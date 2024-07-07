use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

use crate::{
    components::{Damage, Health, Projectile},
    labels::events::EntityKilled,
};

pub fn handle_collisions(
    mut entity_killed_event_writer: EventWriter<EntityKilled>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut contact_event_reader: EventReader<ContactForceEvent>,
    mut damaged_query: Query<&mut Health>,
    damager_query: Query<&Damage>,
    mut projectile_query: Query<&mut Projectile>,
) {
    for collision_event in collision_event_reader.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flag) => {
                info!("Collision Event Started: {:?}, {:?}", e1, e2);

                // Perform collision from e1 -> e2 and e2 -> e1 so both have the others damage applied
                for (e1, e2) in [(*e1, *e2), (*e2, *e1)] {
                    // If e1 has health and e2 deals damage, apply e2 damage to e1 health
                    if let (Ok(mut health), Ok(damage)) =
                        (damaged_query.get_mut(e1), damager_query.get(e2))
                    {
                        // If the damager is a projectile, only want to damage if it has "hits" left
                        if let Ok(mut projectile) = projectile_query.get_mut(e2) {
                            if projectile.add_hit() {
                                health.damage(damage.amount)
                            }
                        } else {
                            health.damage(damage.amount)
                        }

                        // Entity taking damage has no more health, send signal
                        if health.is_zero() {
                            entity_killed_event_writer.send(EntityKilled(e1, e2));
                        }
                    }
                }
            }

            _ => (),
        }
    }

    for _contact_event in contact_event_reader.read() {}
}
