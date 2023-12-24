use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{Motion, Player, Weapon},
    labels::{events::WeaponFired, CursorCoordinates, MainCamera},
};

pub fn get_player_ship_input(
    mut player_query: Query<(&mut Motion, &Weapon, Entity), With<Player>>,
    mut weapon_fired: EventWriter<WeaponFired>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) {
    for (mut motion, weapon, entity) in player_query.iter_mut() {
        let mut direction: Vec2 = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        motion.direction = Quat::from_rotation_arc_2d(Vec2::X, direction.normalize_or_zero());
        motion.is_moving = direction != Vec2::ZERO;

        if buttons.pressed(MouseButton::Left) {
            // Left mouse button was pressed
            if weapon.fire_rate.finished() {
                println!("Firing player weapon");
                // hasn't been too quick since last press
                weapon_fired.send(WeaponFired(entity));
            }
        }
    }
}

pub fn update_cursor_position(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // need to get window dimensions for mouse position
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<&mut Transform, With<CursorCoordinates>>,
) {
    // assumes only one camera has been given the MainCamera component
    let (camera, camera_transform) = camera_query.single();

    let Ok(wnd) = window_query.get_single() else {
        println!("No window found");
        return;
    };

    let Ok(mut cursor_transform) = cursor_query.get_single_mut() else {
        println!("No cursor found");
        return;
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = wnd
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_transform.translation = world_position.extend(0.0);
    }
}
