use bevy::prelude::*;

use crate::{
    labels::{GamePlaySet, GameState, MainSet},
    systems,
};

use super::{PhysicsPlugin, PlayerInputPlugin};

pub struct MainShipGameplayPlugin;

impl Plugin for MainShipGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (MainSet::GamePlay.run_if(in_state(GameState::Playing)),),
        )
        .add_plugins(PlayerInputPlugin)
        // should run player input before all other update systems
        .add_systems(
            Update,
            (
                // We run weapon systems in a strict order here
                (
                    systems::projectile::tick_weapon_fire_rate,
                    systems::projectile::fire_weapon_constantly,
                    systems::projectile::spawn_projectiles_from_weapons_fired,
                )
                    .chain(),
                // systems we are okay with running in parallel during main loop
                (
                    systems::tracking,
                    systems::update_tracking,
                    systems::follow,
                    systems::boundary_position_system,
                    systems::projectile::despawn_projectiles,
                    systems::handle_death,
                    systems::add_score,
                    systems::ui::update_player_ui,
                ),
            )
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Simulation)
                .after(GamePlaySet::Input),
        )
        .add_plugins(PhysicsPlugin)
        // apply collision detection last, after all translations to entities have completed
        .add_systems(
            Update,
            systems::collision
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Collision)
                .after(GamePlaySet::Physics),
        );
    }
}
