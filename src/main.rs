mod movement;
mod rocket;
mod debug;
mod camera;


use bevy::prelude::*; //дает нам доступ к дефолтным плагинам
use rocket::RocketPlugin;
use movement::MovementPlugin;
use debug::DebugPlugin;
use crate::camera::CameraPlugin;

//szx яыч SZX
fn main(){
    App::new() //Расписание
        //Встроенные плагины Bevy
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
        .insert_resource(AmbientLight{
                color: Color::default(),
                brightness: 0.75,
        })
        // пользовательские плагины
        .add_plugins(RocketPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)//обновление и вывод данных

        .run();
}





