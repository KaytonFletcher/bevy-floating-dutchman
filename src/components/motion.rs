use bevy::{
    math::{Mat3, Vec2},
    prelude::{Component, Quat},
};

#[derive(Clone, Component)]
pub struct Motion {
    pub direction: Quat,
    pub is_moving: bool,
    pub acceleration: f32,
    pub max_vel: f32,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            direction: Quat::IDENTITY,
            is_moving: true,
            acceleration: 200.0,
            max_vel: 20.0,
        }
    }
}

impl Motion {
    /**
    * This makes assumptions about our 2D orientation,
      since the Quat holds 3D rotational information
    */
    pub fn get_direction_as_vec2(&self) -> Vec2 {
        let rotation_matrix = Mat3::from_quat(self.direction);
        rotation_matrix.x_axis.truncate().normalize()
    }
}
