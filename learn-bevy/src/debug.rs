use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print);
    }
}

fn print(mut query: Query<(Entity, &mut Transform)>) {
    for (ent, trans) in query.iter_mut() {
        println!("Entity {:?} is at position {:?}", ent, trans.translation)
    }
}
