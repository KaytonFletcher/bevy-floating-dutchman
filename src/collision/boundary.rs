use bevy::{prelude::*, window::PrimaryWindow};

use bevy_rapier2d::prelude::{RigidBody, Velocity};

pub fn wrap_around_screen_edge(
    mut query: Query<
        (&mut Transform, &Velocity),
        (With<RigidBody>, Or<(Changed<Transform>, Changed<Velocity>)>),
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    // Wrap around screen edges
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, velocity) in &mut query.iter_mut() {
        if transform.translation.x < -half_width && velocity.linvel.x < 0.0 {
            transform.translation.x = half_width;
        } else if transform.translation.x > half_width && velocity.linvel.x > 0.0 {
            transform.translation.x = -half_width;
        }

        if transform.translation.y < -half_height && velocity.linvel.y < 0.0 {
            transform.translation.y = half_height;
        } else if transform.translation.y > half_height && velocity.linvel.y > 0.0 {
            transform.translation.y = -half_height;
        }
    }
}
