use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::{Accerleration, MovingObjBundle, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0., -10.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0., 1.);

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn);
    }
}

fn spawn(mut commands: Commands, scene_asset: Res<SceneAssets>) {
    commands.spawn(MovingObjBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        accelearion: Accerleration { value: Vec3::ZERO },
        model: SceneBundle {
            scene: scene_asset.car.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
