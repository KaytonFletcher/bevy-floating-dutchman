use bevy::prelude::*;

use crate::{
    components::{Health, Player},
    labels::GameState,
    resources::UIAssets,
};

#[derive(Component)]
pub struct Heart {
    id: f32,
    half: bool,
}

pub fn spawn_player_ui(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    ui_assets: Res<UIAssets>,
) {
    let num_hearts = 4;

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..num_hearts {
                let left = Val::Px((80.0 * (i as f32)) + 10.0);
                let top = Val::Px(10.0);
     
                parent
                    .spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            position_type: PositionType::Absolute,
                            left,
                            top,
                            ..Default::default()
                        },
                        visibility: Visibility::Visible,
                        image: UiImage {
                            texture: ui_assets.heart_half.clone(),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(Heart {
                        id: i as f32 + 0.5,
                        half: true,
                    });

                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        height: Val::Px(100.0),
                        position_type: PositionType::Absolute,
                        left,
                        top,
                        ..Default::default()
                    },
                    image: UiImage {
                        texture: ui_assets.heart_outline.clone(),
                        ..default()
                    },
                    ..Default::default()
                });

                parent
                    .spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            position_type: PositionType::Absolute,
                            left,
                            top,
                            ..Default::default()
                        },
                        image: UiImage {
                            texture: ui_assets.heart_full.clone(),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(Heart {
                        id: i as f32 + 1.0,
                        half: false,
                    });
            }
        });
    game_state.set(GameState::SpawnEnemies);
}

pub fn update_player_ui(
    mut hearts: Query<(&mut Visibility, &Heart)>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    for health in player_query.iter() {
        for (mut visible, heart) in hearts.iter_mut() {
            // all half and full hearts exist in the world and this logic determines which get drawn
            // at most one half heart of the players health will be visible (the last half)
            // the rest of the health will be full hearts displayed
            *visible = if (heart.id == health.current_health)
                || (heart.id < health.current_health && !heart.half)
            {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
    }
}
