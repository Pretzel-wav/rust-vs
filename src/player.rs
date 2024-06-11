use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.0);

#[derive(Bundle)]
struct PlayerBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Player.glb#Scene0"), //TODO: Change to 2D art, not 3D model
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
