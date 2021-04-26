use crate::components::Player;
use bevy::prelude::*;

pub fn animate_player(mut query: Query<(&Player, &Sprite)>) {
    for (player, sprite) in query.iter_mut() {
        print!("{:?}", player.state);
    }
}
