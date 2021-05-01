use bevy::prelude::*;

use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::{ColliderBuilder, InteractionGroups},
};

pub struct Boundary;

pub fn spawn_room_boundary(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(ColorMaterial::color(Color::BLACK)),
            sprite: Sprite::new(Vec2::new(1000.0, 1000.0)),
            visible: Visible {
                is_transparent: true,
                is_visible: false,
            },
            ..Default::default()
        })
        .insert(RigidBodyBuilder::new_dynamic())
        .insert(
            ColliderBuilder::cuboid(500.0, 500.0)
                .collision_groups(InteractionGroups::new(0b0001, 0b0010)),
        )
        .insert(Boundary);
}
