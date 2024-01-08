use bevy::prelude::*;
use bevy::app::{App, Plugin, Startup};
use bevy::ecs::system::Command;
use bevy::math::Vec3;
use bevy::prelude::{Commands, SpatialBundle};
use crate::movement::Velocity;
//szx яыч SZX

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,-20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0,0.0,1.0);

#[derive(Bundle)]
struct RocketBundle{
    velocity: Velocity,
    model: SceneBundle,
}
pub struct RocketPlugin;

impl Plugin for crate::RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rocket);
    }
}

fn spawn_rocket(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(RocketBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
