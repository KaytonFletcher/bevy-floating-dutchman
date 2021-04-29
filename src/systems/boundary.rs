use bevy::prelude::*;

use bevy_rapier2d::{
    physics::ColliderHandleComponent,
    rapier::{dynamics::RigidBodySet, geometry::ColliderSet},
};

pub fn boundary_system(
    query: Query<(&ColliderHandleComponent)>,
    mut colliders: ResMut<ColliderSet>,
) {
    for (collider) in query.iter() {
        if let Some(c) = colliders.get_mut(collider.handle()) {}
    }
}
