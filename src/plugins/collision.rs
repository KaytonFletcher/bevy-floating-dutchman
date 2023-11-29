use bevy::prelude::*;

use crate::{
    labels::{GamePlaySet, MainSet},
    systems,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // apply collision detection last, after all translations to entities have completed
        app.add_systems(
            Update,
            (systems::collision)
                .in_set(MainSet::GamePlay)
                .in_set(GamePlaySet::Collision),
        );
    }
}
