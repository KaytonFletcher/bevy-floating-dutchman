use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Time, Without};
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{
    components::{Player, Projectile, Track, Weapon},
    entities::spawn_projectile,
    events::WeaponFired,
};

pub fn weapon_fire_rate(mut weapon_query: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in weapon_query.iter_mut() {
        weapon.fire_rate.tick(time.delta());
    }
}

pub fn constant_weapon_fire(
    mut query: Query<(&mut Weapon, Entity), Without<Player>>,
    mut weapons_fired: EventWriter<WeaponFired>,
) {
    for (weapon, entity) in query.iter_mut() {
        if weapon.fire_rate.finished() {
            // hasn't been too quick since last press
            weapons_fired.send(WeaponFired { entity });
        }
    }
}

pub fn weapon_fired(
    mut commands: Commands,
    mut query: Query<(&mut Weapon, &Track, &RigidBodyHandleComponent)>,
    mut weapons_fired: EventReader<WeaponFired>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for event in weapons_fired.iter() {
        if let Ok((mut weapon, track, rb_handle)) = query.get_mut(event.entity) {
            let rb = rigid_bodies.get(rb_handle.handle()).unwrap();
            spawn_projectile(&mut commands, rb, &weapon, track.get_offset());
            weapon.fire_rate.reset();
        }
    }
}

pub fn despawn_projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in query.iter_mut() {
        projectile.time_to_live.tick(time.delta());
        if projectile.time_to_live.finished() {
            commands.entity(entity).despawn();
        }
    }
}
