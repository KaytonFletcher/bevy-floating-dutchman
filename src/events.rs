use bevy::prelude::Entity;

use crate::components::{Damage, Health};

pub struct WeaponFired(pub Entity);

pub struct DamageEvent(Entity, Damage, Health);
