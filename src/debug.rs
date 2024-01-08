use bevy::prelude::*;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{Entity, Query, Transform};


pub struct DebugPlugin;

impl Plugin for crate::DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}
fn print_position(query: Query<(Entity, &Transform)>) {
    //регистрирую ID сущности и Position каждого объекта с помощью компонента "Position"
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation);
    }
}