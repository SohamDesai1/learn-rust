use bevy::prelude::*;

const CAM_DIST: f32 = 60.0;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cam);
    }
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., CAM_DIST, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}
