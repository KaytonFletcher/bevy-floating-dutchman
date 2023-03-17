use bevy::prelude::Bundle;
use bevy_rapier2d::prelude::*;

use super::Health;

#[derive(Bundle, Clone)]
pub struct EnemyBundle {
    pub health: Health,
    // Required for entities we wish to apply a force to
    // Also modified directly to keep entities under their max speed
    pub velocity: Velocity,
    // This is what we modify when moving entities. Because the game is in space,
    // applying forces feels more realistic than modifying velocity directly
    pub force: ExternalForce,
    pub rigid_body: RigidBody,
    pub collider: Collider,

    // This is how we actually read the mass of the collider in systems
    pub read_mass: ReadMassProperties,
    collision_groups: CollisionGroups,
    // This is used to auto-generate mass based on collider shape
    // mass * acceleration is used to apply forces to move enemies
    collider_mass: ColliderMassProperties,
    collision_events: ActiveEvents,
}

pub struct EnemyBuilder {
    pub health: Health,
    // Required for entities we wish to apply a force to
    // Also modified directly to keep entities under their max speed
    pub velocity: Velocity,
    pub collider: Collider,
}

impl EnemyBuilder {
    pub fn new(width: f32, height: f32) -> EnemyBuilder {
        EnemyBuilder {
            health: Health::new(4.0),
            // starting velocity will usually be zero and we apply forces based on Motion component
            // if we don't add motion component, enemy will never move even with this added
            velocity: Velocity::zero(),
            collider: Collider::cuboid(width / 2.0, height / 2.0),
        }
    }

    pub fn with_velocity(mut self, vel: Velocity) -> EnemyBuilder {
        self.velocity = vel;
        self
    }

    pub fn with_health(mut self, health: f32) -> EnemyBuilder {
        self.health = Health::new(health);
        self
    }

    pub fn build(self) -> EnemyBundle {
        EnemyBundle {
            health: self.health,
            velocity: self.velocity,
            collider: self.collider,
            force: ExternalForce::default(),
            rigid_body: RigidBody::Dynamic,
            collision_groups: CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            read_mass: ReadMassProperties::default(),
            collider_mass: ColliderMassProperties::default(),
            collision_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}
