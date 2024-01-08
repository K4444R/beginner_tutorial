use bevy::prelude::*;
use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Query, SpatialBundle, Transform};


#[derive(Component, Debug)]
pub struct Velocity{
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for crate::MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}
fn update_position (mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>){ //обновляем позицию (примечание, делаем изменяемым Position
    for (velocity, mut transform) in query.iter_mut(){ //изменяем через запрос
        transform.translation += velocity.value * time.delta_seconds();
    }
}