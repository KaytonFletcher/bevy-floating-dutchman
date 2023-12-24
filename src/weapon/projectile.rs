use crate::{
    components::{Player, Projectile, Track, Weapon},
    entities::spawn_projectile,
    labels::events::WeaponFired,
};

use bevy::prelude::*;

/**
 * Processes WeaponFired event stream, spawning projectiles for each weapon in world space that has been fired
 */
pub fn spawn_projectiles_from_weapons_fired(
    mut commands: Commands,
    mut query: Query<(&mut Weapon, &Track, &Transform)>,
    player_query: Query<Entity, With<Player>>,
    mut weapons_fired: EventReader<WeaponFired>,
) {
    for WeaponFired(entity) in weapons_fired.read() {
        if let Ok((mut weapon, track, transform)) = query.get_mut(*entity) {
            println!("Spawning projectile for entity: {:?}", entity);
            spawn_projectile(
                &mut commands,
                &transform,
                &weapon,
                track.get_offset(),
                player_query.get(*entity).ok(),
            );
            weapon.fire_rate.reset();
        }
    }
}

/**
 * For each projectile in world space, we check if time to live has expired and then despawn it
 */
pub fn despawn_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in query.iter_mut() {
        projectile.time_to_live.tick(time.delta());
        if projectile.time_to_live.finished() {
            info!("Projectile Killed {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
