use crate::{components::Player, entities::Motion, systems::MainCamera};

use bevy_rapier2d::{
    na::UnitComplex, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};

use bevy::prelude::*;

pub fn player_movement(
    mut queries: QuerySet<(
        Query<(&Player, &mut Motion, &RigidBodyHandleComponent)>,
        Query<&Transform, With<MainCamera>>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut evr_cursor: EventReader<CursorMoved>,
    // need to get window dimensions
    windows: Res<Windows>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let camera_transform = queries.q1().iter().next().unwrap().clone();

    for (_, mut motion, rigid_body) in queries.q0_mut().iter_mut() {
        let mut accel = false;
        if keyboard_input.pressed(KeyCode::A) {
            motion.acceleration.x = -motion.max_accel;
            accel = true;
        }

        if keyboard_input.pressed(KeyCode::D) {
            motion.acceleration.x = motion.max_accel;
            accel = true;
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

        for ev in evr_cursor.iter() {
            // get the size of the window that the event is for
            let wnd = windows.get(ev.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = ev.position - size / 2.0;

            // apply the camera transform
            let mouse_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
                let mut pos = rb.position().clone();

                let rad = (mouse_pos.y - pos.translation.y).atan2(mouse_pos.x - pos.translation.y)
                    + (std::f32::consts::PI / 2.);

                pos.rotation = UnitComplex::new(rad);

                rb.set_position(pos, true);
            }
        }
    }
}
