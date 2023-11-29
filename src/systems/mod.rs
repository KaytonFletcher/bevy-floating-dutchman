mod boundary;
mod collision;
pub mod debug;
mod handle_death;
mod movement;
mod player;
pub mod projectile;
mod setup;

pub mod ui;

pub use boundary::boundary_position_system;
pub use collision::collision;
pub use handle_death::add_score;
pub use handle_death::handle_death;
pub use movement::follow;
pub use movement::*;
pub use player::player_input;
pub use setup::setup;
pub use setup::MainCamera;
