pub use bevy::prelude::*;

use crate::{
    components::{Player, Weapon},
    labels::events::WeaponFired,
};

pub fn tick_weapon_fire_rate(mut weapon_query: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in weapon_query.iter_mut() {
        weapon.fire_rate.tick(time.delta());
    }
}

/**
 * Certain enemies will rely on this system to fire their weapon constantly
 */
pub fn fire_weapon_constantly(
    mut query: Query<(&mut Weapon, Entity), Without<Player>>,
    mut weapons_fired: EventWriter<WeaponFired>,
) {
    for (weapon, entity) in query.iter_mut() {
        if weapon.fire_rate.finished() {
            // always fire at steady fire rate
            weapons_fired.send(WeaponFired(entity));
        }
    }
}
