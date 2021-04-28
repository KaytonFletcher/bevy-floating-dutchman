use crate::{components::Player, entities::Motion, systems::MainCamera};

use bevy_rapier2d::rapier::dynamics::JointSet;

use bevy::prelude::*;

pub fn player_movement(
    mut queries: QuerySet<(
        Query<(
            &Player,
            &mut Transform,
            &mut Motion,
        )>,
        Query<&Transform, With<MainCamera>>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    windows: Res<Windows>
) {
    let camera_transform = queries.q1().iter().next().unwrap().clone();

    for (_, mut transform, mut motion) in queries.q0_mut().iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            motion.acceleration.x = -motion.max_accel;
        }

        if keyboard_input.pressed(KeyCode::D) {
            motion.acceleration.x = motion.max_accel;
        }

        if keyboard_input.pressed(KeyCode::W) {
            motion.acceleration.y = motion.max_accel;
        }

        if keyboard_input.pressed(KeyCode::S) {
            motion.acceleration.y = -motion.max_accel;
        }

        let player_pos = transform.translation;

        for ev in evr_cursor.iter() {
            // get the size of the window that the event is for
            let wnd = windows.get(ev.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = ev.position - size / 2.0;

            // apply the camera transform
            let mouse_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            let rad = (mouse_pos.y - player_pos.y).atan2(mouse_pos.x - player_pos.x)
                + (std::f32::consts::PI / 2.);

            transform.rotation = Quat::from_axis_angle(Vec3::Z, rad);
        }
    }
}
