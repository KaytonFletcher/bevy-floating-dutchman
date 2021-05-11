use bevy::prelude::{Commands, Entity, Query, Res, Time};

use crate::components::{Projectile, Weapon};

pub fn weapon_fire_rate(mut weapon_query: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in weapon_query.iter_mut() {
        weapon.fire_rate.tick(time.delta());
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
