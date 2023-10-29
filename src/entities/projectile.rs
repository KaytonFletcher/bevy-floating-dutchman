use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Weapon;

pub fn spawn_projectile(
    commands: &mut Commands,
    transform: &Transform,
    weapon: &Weapon,
    ang_offset: f32,
) {
    let mut new_pb = weapon.projectile.clone();

    // ang_offset allows for spawning a projectile moving in some direction
    // offset from the orientation of the entity "firing" the projectile.
    // Can also be used to account for sprites on the "firing" entity that have some necessary offset
    let angle = transform.rotation * Quat::from_rotation_z(-ang_offset);

    let direction = Mat3::from_quat(angle).x_axis;

    // This represents how far (weapon.pos_offset) and direction
    // from center of weapon-holder to spawn projectile
    let start_pos_offset = direction * weapon.pos_offset;

    // At a high level, ang_offset + start_pos_offset gives full control on starting
    // location and direction to fire a projectile, relative to the transform provided
    new_pb.sprite.transform.translation = transform.translation + start_pos_offset;

    println!("Projectile angle: {}", angle);

    let mut entity_builder = commands.spawn(new_pb);

    entity_builder
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            angvel: 0.0,
            linvel: direction.truncate() * 600.0,
        });
}
