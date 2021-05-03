use bevy::prelude::*;

use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod components;
mod entities;
mod resources;
mod systems;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "The Floating Dutchman".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(bevy::winit::WinitPlugin::default())
        // .add_plugin(bevy::wgpu::WgpuPlugin::default())
        .add_startup_system(systems::setup.system())
        .add_startup_system(entities::init_player.system())
        // .add_startup_system(entities::spawn_room_boundary.system())
        .add_system(systems::player_movement.system())
        .add_system(systems::player_dampening.system())
        .add_system(systems::movement.system())
        .add_plugin(RapierPhysicsPlugin)
        // .add_plugin(RapierRenderPlugin)
        .run();
}
