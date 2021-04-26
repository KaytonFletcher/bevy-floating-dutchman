#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Idling,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Idling
    }
}

pub struct Player {
    pub state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::default(),
        }
    }
}
