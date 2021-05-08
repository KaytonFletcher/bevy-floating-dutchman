use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{EventQueue, RigidBodyHandleComponent},
    rapier::{
        dynamics::RigidBodySet,
        geometry::{ColliderSet, ContactEvent},
    },
};

use crate::components::{Health, Player};

pub fn collision(
    mut commands: Commands,
    mut player_query: Query<&mut Health, With<Player>>,
    events: Res<EventQueue>,
    rigid_bodies: ResMut<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
    // handles: Query<&RigidBodyHandleComponent>,
) {
    // let mut contacts = vec![];
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let c1 = colliders.get(h1).unwrap();
                let c2 = colliders.get(h2).unwrap();
                let b1 = rigid_bodies.get(c1.parent()).unwrap();
                let b2 = rigid_bodies.get(c2.parent()).unwrap();
                let e1 = Entity::from_bits(b1.user_data as u64);
                let e2 = Entity::from_bits(b2.user_data as u64);

                // println!("CONTACT STARTED");

                if player_query.get_component::<Health>(e1).is_ok() {
                    // println!("DELETE");

                    commands.entity(e2).despawn();
                } else if player_query.get_component::<Health>(e2).is_ok() {
                    commands.entity(e1).despawn();
                }
            }
            _ => (),
        }
    }
}
