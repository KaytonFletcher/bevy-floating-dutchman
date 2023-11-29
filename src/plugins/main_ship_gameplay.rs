use bevy::prelude::*;

use crate::{
    labels::{GamePlaySet, GameState, MainSet},
    systems,
};

use super::{CollisionPlugin, PhysicsPlugin, PlayerInputPlugin};

pub struct MainShipGameplayPlugin;

impl Plugin for MainShipGameplayPlugin {
    fn build(&self, app: &mut App) {
        // Configuring the ordering of our gameplay loop using these main sets:
        // Input -> Simulation -> Physics -> Collision
        app.configure_sets(
            Update,
            (
                MainSet::GamePlay.run_if(in_state(GameState::Playing)),
                GamePlaySet::Input.before(GamePlaySet::Simulation),
                GamePlaySet::Simulation.before(GamePlaySet::Physics),
                GamePlaySet::Physics.before(GamePlaySet::Collision),
            ),
        )
        .add_plugins(PlayerInputPlugin)
        .add_systems(
            Update,
            (
                (
                    // We run weapon systems in a strict order here
                    (
                        systems::projectile::tick_weapon_fire_rate,
                        systems::projectile::fire_weapon_constantly,
                        systems::projectile::spawn_projectiles_from_weapons_fired
                            .after(systems::update_rotation_based_on_tracking),
                    )
                        .chain(),
                    // systems we are okay with running in parallel during main loop
                    (
                        (
                            systems::update_position_of_entity_tracked,
                            systems::update_rotation_based_on_tracking,
                        )
                            .chain(),
                        systems::follow.ambiguous_with(systems::update_rotation_based_on_tracking),
                        systems::projectile::despawn_projectiles,
                        systems::handle_death,
                        systems::add_score,
                        systems::ui::update_player_ui.after(systems::add_score),
                    ),
                ),
                // Apply boundary corrective system after all other translations
                systems::boundary_position_system,
            )
                .chain()
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Simulation),
        )
        .add_plugins((PhysicsPlugin, CollisionPlugin));
    }
}
