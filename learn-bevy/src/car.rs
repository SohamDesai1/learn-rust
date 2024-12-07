use bevy::prelude::*;

use crate::movement::{Accerleration, MovingObjBundle, Velocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0., -10.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0., 1.);

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovingObjBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        accelearion: Accerleration { value: Vec3::ZERO },
        model: SceneBundle {
            scene: asset_server.load("race.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
