use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut trans) in query.iter_mut() {
        trans.translation += vel.value * time.delta_seconds();
    }
}
