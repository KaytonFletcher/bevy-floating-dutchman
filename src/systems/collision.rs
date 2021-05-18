use bevy::prelude::*;
use bevy_rapier2d::{
    physics::EventQueue,
    rapier::{
        dynamics::RigidBodySet,
        geometry::{ColliderSet, ContactEvent},
    },
};

use crate::components::{Damage, Health};

pub fn collision(
    mut commands: Commands,
    mut player_query: Query<&mut Health>,
    damage_query: Query<&Damage>,
    events: Res<EventQueue>,
    rigid_bodies: ResMut<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let c1 = colliders.get(h1).unwrap();
                let c2 = colliders.get(h2).unwrap();
                let b1 = rigid_bodies.get(c1.parent()).unwrap();
                let b2 = rigid_bodies.get(c2.parent()).unwrap();
                let e1 = Entity::from_bits(b1.user_data as u64);
                let e2 = Entity::from_bits(b2.user_data as u64);

                if let Ok(mut health) = player_query.get_component_mut::<Health>(e1) {
                    if let Ok(damage) = damage_query.get_component::<Damage>(e2) {
                        if health.damage(damage.amount) {
                            commands.entity(e1).despawn();
                        }
                    }
                }

                if let Ok(mut health) = player_query.get_component_mut::<Health>(e2) {
                    if let Ok(damage) = damage_query.get_component::<Damage>(e1) {
                        if health.damage(damage.amount) {
                            commands.entity(e2).despawn();
                        }
                    }
                }
            }
            _ => (),
        }
    }

    while let Ok(intersect_event) = events.intersection_events.pop() {
        if intersect_event.intersecting {
            println!("intersection event");

            let c1 = colliders.get(intersect_event.collider1).unwrap();
            let c2 = colliders.get(intersect_event.collider2).unwrap();
            let b1 = rigid_bodies.get(c1.parent()).unwrap();
            let b2 = rigid_bodies.get(c2.parent()).unwrap();
            let e1 = Entity::from_bits(b1.user_data as u64);
            let e2 = Entity::from_bits(b2.user_data as u64);

            if let Ok(mut health) = player_query.get_component_mut::<Health>(e1) {
                if let Ok(damage) = damage_query.get_component::<Damage>(e2) {
                    println!("damage");

                    if health.damage(damage.amount) {
                        commands.entity(e1).despawn();
                    }
                }
            }

            if let Ok(mut health) = player_query.get_component_mut::<Health>(e2) {
                if let Ok(damage) = damage_query.get_component::<Damage>(e1) {
                    println!("damage 2");

                    if health.damage(damage.amount) {
                        commands.entity(e2).despawn();
                    }
                }
            }
        }
    }
}
