use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Range;

use crate::movement::{Accerleration, MovingObjBundle, Velocity};

const SPAWN_RANGE_X: Range<f32> = -20.0..20.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..20.0;
const VEL: f32 = 5.;
const ACCEL: f32 = 1.;
const SPAWN_TIME: f32 = 2.;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_obstacle);
    }
}
fn spawn_obstacle(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    let mut rand = thread_rng();

    let trans = Vec3::new(
        rand.gen_range(SPAWN_RANGE_X),
        0.,
        rand.gen_range(SPAWN_RANGE_Z),
    );

    let mut rand_unit_vec =
        || Vec3::new(rand.gen_range(-1.0..1.0), 0., rand.gen_range(-1.0..1.0)).normalize_or_zero();

    let vel = rand_unit_vec() * VEL;
    let accel = rand_unit_vec() * ACCEL;

    commands.spawn((
        MovingObjBundle {
            velocity: Velocity::new(vel),
            accelearion: Accerleration::new(accel),
            model: SceneBundle {
                scene: asset_server.load("debris-bumper.glb#Scene0"),
                transform: Transform::from_translation(trans),
                ..default()
            },
        },
        Obstacle,
    ));
}
