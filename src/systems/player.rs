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
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
        
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = wnd.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            track_mouse.pos = world_position;
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
