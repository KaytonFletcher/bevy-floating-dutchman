use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::labels::states::GameState;

#[derive(Component)]
pub struct PauseMenu;

pub fn create_pause_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::BLACK.with_alpha(0.9).into(), // want to allow game to be seen in background
            visibility: Visibility::Hidden, // created pause menu should be hidden to start
            ..Default::default()
        })
        // render this above in-game UI such as player health and score
        .insert(ZIndex::Global(1))
        .insert(PauseMenu)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Game Paused",
                    TextStyle {
                        font_size: 70.0,
                        ..default()
                    },
                )
                .with_style(Style {
                    top: Val::Percent(-20.0),
                    ..default()
                }),
            );
        });
}

// Make pause menu visible when we enter the state
pub fn on_pause(
    mut pause_menu_query: Query<&mut Visibility, With<PauseMenu>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if let Ok(mut pause_menu_visibility) = pause_menu_query.get_single_mut() {
        *pause_menu_visibility = Visibility::Visible;
    }

    rapier_config.physics_pipeline_active = false;
}

// Cleanup pause menu once we return to game, set it to hidden
pub fn on_resume_game(
    mut pause_menu_query: Query<&mut Visibility, With<PauseMenu>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if let Ok(mut pause_menu_visibility) = pause_menu_query.get_single_mut() {
        *pause_menu_visibility = Visibility::Hidden;
    }

    rapier_config.physics_pipeline_active = true;
}

// Watcher system to determine when to go back to the game
pub fn return_to_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Playing);
    }
}
