use crate::{
    components::{Player, Track},
    entities::Motion,
    systems::MainCamera,
};

use bevy_rapier2d::{physics::{RigidBodyHandleComponent, RapierConfiguration}, rapier::dynamics::RigidBodySet};

use bevy::prelude::*;

pub fn player_dampening(
    query: Query<(&Player, &RigidBodyHandleComponent)>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (_, rb_handle) in query.iter() {
        let elapsed = time.delta_seconds();
        let rb = rigid_bodies.get_mut(rb_handle.handle()).unwrap();
        rb.set_angvel(rb.angvel() * 0.005f32.powf(elapsed), false);
        rb.set_linvel(rb.linvel() * 0.8f32.powf(elapsed), false);
    }
}

pub fn player_movement(
    mut queries: QuerySet<(
        Query<(
            &Player,
            &mut Track,
            &mut Motion,
            &RigidBodyHandleComponent,
        )>,
        Query<&Transform, With<MainCamera>>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    // need to get window dimensions for mouse position
    windows: Res<Windows>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    rapier_parameters: Res<RapierConfiguration>,
) {
    let camera_transform = queries.q1().iter().next().unwrap().clone();

    for (_, mut latest_mouse_pos, mut motion, rigid_body) in queries.q0_mut().iter_mut()
    {
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

            // println!("Mouse Position: {:?}", mouse_pos);

            latest_mouse_pos.x = mouse_pos.x;
            latest_mouse_pos.y = mouse_pos.y;
        }

        // always true
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            // angle between player position and last known mouse position
            let mut new_angle = ((latest_mouse_pos.y/rapier_parameters.scale) - rb.position().translation.vector.y)
                .atan2((latest_mouse_pos.x/rapier_parameters.scale) - rb.position().translation.vector.x)
                + (std::f32::consts::PI / 2.);

            // subtracts player angle to get the difference in angles
            new_angle -= rb.position().rotation.angle();

            let f = 60.0;
            let torque = f * new_angle.sin();

            rb.apply_torque_impulse(torque, true);
        }
    }
}
