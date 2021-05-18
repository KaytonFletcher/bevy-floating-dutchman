#[derive(Clone)]
pub struct Damage {
    pub amount: f32,
}

impl Default for Damage {
    fn default() -> Self {
        Damage { amount: 1.0 }
    }
}

#[derive(Clone)]
pub struct Health {
    pub max_health: f32,
    pub current_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            max_health,
            current_health: max_health,
        }
    }

    pub fn heal(&mut self, add_health: f32) {
        self.current_health = (self.current_health + add_health).min(self.max_health);
    }

    // returns true if the damage applied takes health to zero
    pub fn damage(&mut self, amount: f32) -> bool {
        self.current_health -= amount;
        if self.current_health <= 0.0 {
            self.current_health = 0.0;
            true
        } else {
            false
        }
    }
}
