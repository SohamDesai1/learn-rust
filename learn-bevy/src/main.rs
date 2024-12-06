use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, (update, print_posi))
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}

fn print_posi(mut query: Query<(Entity, &mut Position)>) {
    for (ent, posi) in query.iter_mut() {
        println!("Entity {:?} is at position {:?}", ent, posi)
    }
}
