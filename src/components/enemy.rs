use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Health, Motion, Score};

#[derive(Bundle, Clone)]
// This enemy can still have forces affect its position, but it does not move
pub struct StaticEnemyBundle {
    pub health: Health,
    pub score: Score,
    pub sprite_bundle: SpriteBundle,

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

#[derive(Bundle, Clone)]
pub struct MovingEnemyBundle {
    // These two structs act in tandem. Velocity is the speed/direction
    // of the enemy reflected by the physics engine after forces are applied.
    // We also modify it directly to keep entities under their max speed.
    pub velocity: Velocity,
    // Motion is a custom struct for storing where an entity SHOULD move
    // via forces when the physics systems tick again
    // This struct can be expanded to incorporate more complex movement data in the future
    pub motion: Motion,
    pub static_enemy: StaticEnemyBundle,
}

// EnemyBuilder struct contains the enemy fields we plan to change from one enemy to the next,
// other fields can be default constructed to remove boilerplate code
pub struct EnemyBuilder {
    pub health: Health,
    pub score: Score,
    pub motion: Motion,
    pub collider: Collider,
    pub sprite_bundle: SpriteBundle,
}

impl EnemyBuilder {
    pub fn new(size: Vec2, scale: f32, start_pos: Vec2, sprite: Handle<Image>) -> EnemyBuilder {
        EnemyBuilder {
            health: Health::new(4.0),
            score: Score { score: 10 },
            motion: Motion::default(),
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(start_pos.x, start_pos.y, 1.0),
                    scale: Vec3::new(scale, scale, 1.0),
                    ..Default::default()
                },
                texture: sprite,
                ..Default::default()
            },
        }
    }

    pub fn with_motion(mut self, motion: Motion) -> EnemyBuilder {
        self.motion = motion;
        self
    }

    pub fn with_health(mut self, health: f32) -> EnemyBuilder {
        self.health = Health::new(health);
        self
    }

    pub fn with_score(mut self, score: i32) -> EnemyBuilder {
        self.score = Score { score };
        self
    }

    pub fn build(self) -> MovingEnemyBundle {
        MovingEnemyBundle {
            // starting velocity will usually be zero and we apply forces based on Motion component
            // if we don't add motion component, enemy will never move even with this added
            velocity: Velocity::zero(),
            motion: self.motion,
            static_enemy: StaticEnemyBundle {
                health: self.health,
                score: self.score,
                sprite_bundle: self.sprite_bundle,
                collider: self.collider,
                force: ExternalForce::default(),
                rigid_body: RigidBody::Dynamic,
                collision_groups: CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
                read_mass: ReadMassProperties::default(),
                collider_mass: ColliderMassProperties::default(),
                collision_events: ActiveEvents::COLLISION_EVENTS,
            },
        }
    }
}
