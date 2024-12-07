pub mod camera;
pub mod car;
pub mod debug;
pub mod movement;
pub mod obstacle;

use bevy::prelude::*;
use camera::CamPlugin;
use car::CarPlugin;
use debug::DebugPlugin;
use obstacle::ObstaclePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CarPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CamPlugin)
        .add_plugins(ObstaclePlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(AmbientLight {
            brightness: 0.7,
            color: Color::default(),
        })
        .run();
}
