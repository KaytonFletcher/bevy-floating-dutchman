use bevy::prelude::{Commands, EventReader};

use crate::events::{EntityKilled, PlayerKilled};

pub fn handle_death(
    mut commands: Commands,
    mut entities_killed: EventReader<EntityKilled>,
    mut players_killed: EventReader<PlayerKilled>,
) {
    for EntityKilled(e1) in entities_killed.iter() {
        commands.entity(*e1).despawn();
    }

    for PlayerKilled(player) in players_killed.iter() {
        commands.entity(*player).despawn();
    }
}
