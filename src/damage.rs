use bevy::prelude::*;

use crate::{
    components::{Health, Projectile},
    labels::events::{DealDamage, EntityDeath},
};

pub fn on_damage_taken(
    trigger: Trigger<DealDamage>,
    mut commands: Commands,
    mut damaged_query: Query<&mut Health>,
    mut projectile_query: Query<&mut Projectile>,
) {
    if let Ok(mut health) = damaged_query.get_mut(trigger.entity()) {
        // If the damager is a projectile, only want to damage if it has "hits" left
        if let Ok(mut projectile) = projectile_query.get_mut(trigger.entity()) {
            if projectile.add_hit() {
                health.damage(trigger.event().damage)
            }
        } else {
            health.damage(trigger.event().damage)
        }

        // Entity taking damage has no more health, send signal
        if health.is_zero() {
            commands.trigger_targets(
                EntityDeath {
                    cause: trigger.event().cause,
                },
                trigger.entity(),
            );
        }
    }
}
