use bevy::prelude::*;

use crate::{
    labels::GamePlaySet,
    systems::{boundary, death, movement, projectile, score, ui},
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    // Weapon systems run in a strict order
                    (
                        projectile::tick_weapon_fire_rate,
                        projectile::fire_weapon_constantly,
                        projectile::spawn_projectiles_from_weapons_fired
                            .after(movement::update_rotation_based_on_tracking),
                    )
                        .chain(),
                    // Track systems run in a strict order
                    (
                        movement::update_position_of_entity_tracked,
                        movement::update_rotation_based_on_tracking
                            .ambiguous_with(movement::follow),
                    )
                        .chain(),
                    // systems we are okay with running in parallel during main loop
                    (
                        movement::follow,
                        projectile::despawn_projectiles,
                        score::add_scores_from_killed.before(ui::update_player_ui),
                        ui::update_player_ui,
                    ),
                ),
                // These systems run last, in parallel where possible but after all other simulation systems
                (death::handle_death, boundary::boundary_position_system),
            )
                .chain()
                .in_set(GamePlaySet::Simulation),
        );
    }
}
