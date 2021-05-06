use crate::{
    components::{Player, Track},
    entities::Motion,
    systems::MainCamera,
};

use bevy::prelude::*;

use bevy_rapier2d::physics::RapierConfiguration;

pub fn player_movement(
    mut queries: QuerySet<(
        Query<(&mut Track, &mut Motion), With<Player>>,
        Query<&Transform, With<MainCamera>>,
    )>,
    mut evr_cursor: EventReader<CursorMoved>,

    keyboard_input: Res<Input<KeyCode>>,
    // need to get window dimensions for mouse position
    windows: Res<Windows>,
    rapier_parameters: Res<RapierConfiguration>,
) {
    let camera_transform = queries.q1().iter().next().unwrap().clone();

    for (mut track_mouse, mut motion) in queries.q0_mut().iter_mut() {
        let mut accel = false;
        if keyboard_input.pressed(KeyCode::A) {
            motion.acceleration.x = -motion.max_accel;
            accel = true;
        }

        if keyboard_input.pressed(KeyCode::D) {
            if accel {
                motion.acceleration.x = 0.;
            } else {
                motion.acceleration.x = motion.max_accel;
                accel = true;
            }
        }

        if !accel {
            motion.acceleration.x = 0.0;
        }

        accel = false;

        if keyboard_input.pressed(KeyCode::W) {
            motion.acceleration.y = motion.max_accel;
            accel = true;
        }

        if keyboard_input.pressed(KeyCode::S) {
            motion.acceleration.y = -motion.max_accel;
            accel = true;
        }

        if !accel {
            motion.acceleration.y = 0.0;
        }

        // has a new mouse event occured
        if let Some(cursor_event) = evr_cursor.iter().next_back() {
            // get the size of the window that the event is for
            let wnd = windows.get(cursor_event.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = cursor_event.position - size / 2.0;

            // apply the camera transform
            let mouse_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            // update the position the player is tracking (rotating towards mouse pos)
            track_mouse.pos = (mouse_pos / rapier_parameters.scale).into();
        }
    }
}
