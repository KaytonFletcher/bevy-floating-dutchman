use crate::{
    components::{Player, Projectile, Track, Weapon},
    entities::spawn_projectile,
    events::WeaponFired,
};
use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Query, Res, Time, Transform, Without,
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
    mut query: Query<(&mut Weapon, &Track, &Transform)>,
    mut weapons_fired: EventReader<WeaponFired>,
) {
    for event in weapons_fired.iter() {
        if let Ok((mut weapon, track, transform)) = query.get_mut(event.entity) {
            spawn_projectile(&mut commands, &transform, &weapon, track.get_offset());
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
