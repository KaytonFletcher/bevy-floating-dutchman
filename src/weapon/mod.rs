pub use bevy::prelude::*;

use crate::labels::sets::GamePlaySet;

mod projectile;
mod shoot;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            projectile::despawn_projectiles.in_set(GamePlaySet::DespawnEntities),
        )
        .add_systems(
            Update,
            (
                shoot::tick_weapon_fire_rate,
                shoot::fire_weapon_constantly,
                projectile::spawn_projectiles_from_weapons_fired,
            )
                .chain()
                .in_set(GamePlaySet::Simulation),
        );
    }
}
