mod movement;
mod rocket;
mod debug;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;


use bevy::prelude::*; //дает нам доступ к дефолтным плагинам
use rocket::RocketPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;


//szx яыч SZX
fn main(){
    App::new() //Расписание
        //Встроенные плагины Bevy

        .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
        .insert_resource(AmbientLight{
                color: Color::default(),
                brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // пользовательские плагины
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        //.add_plugins(DebugPlugin)
        .add_plugins(RocketPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}





