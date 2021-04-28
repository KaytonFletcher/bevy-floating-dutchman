use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::components::collisions::{Boundary, Collider};

pub struct BoundarySystem;

impl<'s> System<'s> for BoundarySystem {
    type SystemData = (WriteStorage<'s, Collider>, ReadStorage<'s, Boundary>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut colliders, boundaries) = data;

        for (collider, boundary) in (&mut colliders, &boundaries).join() {
            let mut max_x = 0.;
            let mut max_y = 0.;
            let mut min_x = 10000000.;
            let mut min_y = 10000000.;

            for vertex in collider.hit_box.vertices.iter() {
                if vertex.x > max_x {
                    max_x = vertex.x;
                }

                if vertex.x < min_x {
                    min_x = vertex.x
                }

                if vertex.y > max_y {
                    max_y = vertex.y;
                }

                if vertex.y < min_y {
                    min_y = vertex.y
                }
            }

            let correction_x = if max_x > boundary.right {
                max_x - boundary.right
            } else if min_x < boundary.left {
                min_x - boundary.left
            } else {
                0.
            };

            let correction_y = if max_y > boundary.top {
                max_y - boundary.top
            } else if min_y < boundary.bottom {
                min_y - boundary.bottom
            } else {
                0.
            };

            let hbox = &mut collider.hit_box;
            hbox.old_position = hbox.position;
            hbox.position.x -= correction_x;
            hbox.position.y -= correction_y;
        }
    }
}
