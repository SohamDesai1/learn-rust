use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Accerleration {
    pub value: Vec3,
}

impl Accerleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjBundle {
    pub velocity: Velocity,
    pub accelearion: Accerleration,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position));
    }
}

fn update_velocity(mut query: Query<(&Accerleration, &mut Velocity)>, time: Res<Time>) {
    for (acc, mut vel) in query.iter_mut() {
        vel.value += acc.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut trans) in query.iter_mut() {
        trans.translation += vel.value * time.delta_seconds();
    }
}
