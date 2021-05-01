use bevy::prelude::*;

use bevy_rapier2d::{
    physics::{ColliderHandleComponent, EventQueue},
    rapier::{dynamics::RigidBodySet, geometry::ColliderSet},
};

pub fn boundary_system(events: Res<EventQueue>) {
    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }

    while let Ok(intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }
}
