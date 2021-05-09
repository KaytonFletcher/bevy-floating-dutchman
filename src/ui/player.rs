use bevy::prelude::*;

use crate::components::{Health, Player};

pub struct Heart {
    id: f32,
    half: bool,
}

pub fn spawn_player_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let heart_outline_handle = materials.add(asset_server.load("sprites/heart_outline.png").into());
    let full_heart_handle = materials.add(asset_server.load("sprites/full_heart.png").into());
    let half_heart_handle = materials.add(asset_server.load("sprites/half_heart.png").into());

    let num_hearts = 4;

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..num_hearts {
                let pos = Rect {
                    left: Val::Px((80.0 * (i as f32)) + 10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                };

                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                            position_type: PositionType::Absolute,
                            position: pos,
                            ..Default::default()
                        },
                        visible: Visible {
                            is_visible: false,
                            ..Default::default()
                        },
                        material: half_heart_handle.clone(),
                        ..Default::default()
                    })
                    .insert(Heart {
                        id: i as f32 + 0.5,
                        half: true,
                    });

                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        position_type: PositionType::Absolute,
                        position: pos,
                        ..Default::default()
                    },
                    material: heart_outline_handle.clone(),
                    ..Default::default()
                });

                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                            position_type: PositionType::Absolute,
                            position: pos,
                            ..Default::default()
                        },
                        material: full_heart_handle.clone(),
                        ..Default::default()
                    })
                    .insert(Heart {
                        id: i as f32 + 1.0,
                        half: false,
                    });
            }
        });
}

pub fn update_player_ui(
    mut hearts: Query<(&mut Visible, &Heart)>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    for health in player_query.iter() {
        for (mut visible, heart) in hearts.iter_mut() {
            // all half and full hearts exist in the world and this logic determines which get drawn
            // at most one half heart of the players health will be visible (the last half)
            // the rest of the health will be full hearts displayed
            visible.is_visible = (heart.id == health.current_health)
                || (heart.id < health.current_health && !heart.half)
        }
    }
}
