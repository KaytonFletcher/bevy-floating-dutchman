use crate::{
    components::Motion,
    components::{Player, Track},
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
        motion.direction.x = 0.0;
        motion.direction.y = 0.0;

        if keyboard_input.pressed(KeyCode::A) {
            motion.direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            motion.direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            motion.direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            motion.direction.y -= 1.0;
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
