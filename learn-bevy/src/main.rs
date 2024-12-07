pub mod camera;
pub mod car;
pub mod debug;
pub mod movement;
pub mod obstacle;
pub mod asset_loader;

use bevy::prelude::*;
use camera::CamPlugin;
use car::CarPlugin;
use obstacle::ObstaclePlugin;
use asset_loader::AssetLoaderPlugin;
use debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CamPlugin)
        .add_plugins(CarPlugin)
        .add_plugins(ObstaclePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DebugPlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(AmbientLight {
            brightness: 0.7,
            color: Color::default(),
        })
        .run();
}
