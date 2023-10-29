use crate::{
    components::Motion,
    components::{Player, Track, Weapon},
    events::WeaponFired,
    systems::MainCamera,
};

use bevy::{prelude::*, window::PrimaryWindow};

pub fn player_input(
    mut player_query: Query<(&mut Track, &mut Motion, &Weapon, Entity), With<Player>>,
    mut weapon_fired: EventWriter<WeaponFired>,
    camera_query: Query<(&Camera, &Transform), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    // need to get window dimensions for mouse position
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut track_mouse, mut motion, weapon, entity) in player_query.iter_mut() {
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

        // println!("Vec2 direction: {}", direction);
        // println!("Quat direction: {}", motion.direction);

        motion.is_moving = direction != Vec2::ZERO;

        // assumes only one camera has been given the MainCamera component
        let (camera, camera_transform) = camera_query.single();

        // get the window that the camera is displaying to (or the primary window)
        // let wnd = if let RenderTarget::Window(id) = camera.target {
        //     windows.get(id).unwrap()
        // } else {
        //     windows.get_single().unwrap()
        // };

        let wnd = windows.get_single().unwrap();

        // has a new mouse event occured
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value and update the position
            // the player is tracking (rotating towards mouse pos)
            let world_pos: Vec2 = world_pos.truncate();
            if !world_pos.is_nan() && world_pos != Vec2::ZERO {
                track_mouse.pos = world_pos;
            }
        }

        if buttons.pressed(MouseButton::Left) {
            // Left mouse button was pressed
            if weapon.fire_rate.finished() {
                println!("Weapon Fired");
                // hasn't been too quick since last press
                weapon_fired.send(WeaponFired(entity));
            }
        }
    }
}
