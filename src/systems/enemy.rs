use bevy::prelude::*;

use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{
    components::{Player, Track},
    resources::Game,
};

pub fn enemy_tracking(
    mut queries: QuerySet<(
        Query<&mut Track, Without<Player>>,
        Query<&RigidBodyHandleComponent, With<Player>>,
    )>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    game: Res<Game>,
) {
    if let Some(player_entity) = game.player {
        if let Ok(rb_handle) = queries
            .q1()
            .get_component::<RigidBodyHandleComponent>(player_entity)
        {
            let rb = rigid_bodies.get_mut(rb_handle.handle()).unwrap();

            for mut track in queries.q0_mut().iter_mut() {
              track.pos.x = rb.position().translation.x;
              track.pos.y = rb.position().translation.y;
              // println!("Updating position {:?}", track.pos);
            }
        }
    }
}
