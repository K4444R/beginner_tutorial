use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Velocity, MovingObjectBundle, Acceleration};
//szx яыч SZX

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,-20.0);
const ROCKET_SPEED: f32 = 25.0;
const ROCKET_ROTATION_SPEED: f32 = 2.5;
const ROCKET_ROLL_SPEED: f32 = 2.5;
const ROCKET_RADIUS: f32 = 5.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_RADIUS: f32 = 1.0;



#[derive(Component, Debug)]
pub struct Rocket;

#[derive(Component, Debug)]
pub struct RocketMissile;

pub struct RocketPlugin;

impl Plugin for crate::RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_rocket).add_systems(
            Update,
            (rocket_movement_controls, rocket_weapon_controls)
        );
    }
}

fn spawn_rocket(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO) ,
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(ROCKET_RADIUS),
            model: SceneBundle {
                scene: scene_assets.rocket.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Rocket,
    ));
}


fn rocket_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Rocket>>, // With<> Используется для ограничения запросов, чтобы получить другие компоненты которые имеют этот
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    //Назначаем клавищи передвижения
    if keyboard_input.pressed(KeyCode::H){ //Вправо H
        rotation = -ROCKET_ROTATION_SPEED * time.delta_seconds(); // умножаем на time.delta_seconds() чтобы ракета вращалось с постоянной скоростью за кадр.
    } else if keyboard_input.pressed(KeyCode::F){ // Влево F
        rotation = ROCKET_ROTATION_SPEED * time.delta_seconds();
    }


    if keyboard_input.pressed(KeyCode::G){ //Назад G
        movement = -ROCKET_SPEED;
    } else if keyboard_input.pressed(KeyCode::T){  // Вперед T
        movement = ROCKET_SPEED;
    }

    if keyboard_input.pressed(KeyCode::C){ //
        roll = -ROCKET_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::Y){  //
        roll = ROCKET_ROLL_SPEED * time.delta_seconds();
    }

    //Вращается вокруг оси У
    //Игнорирует вращение оси z
    transform.rotate_y(rotation);

    //Вращается вокруг локалной оси z
    //вращение происходит онтносительно текущего вращения
    transform.rotate_local_z(roll);

    //Обновляем скорость рокеты, в зависимости от нового направления
    velocity.value = -transform.forward() * movement; //B Bevy, прямое (вперед) напрвление оси z идет с минусовым значением!!!
}

fn rocket_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Rocket>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
){
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((MovingObjectBundle{
            velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILE_RADIUS),
            model: SceneBundle{
                scene: scene_assets.missiles.clone(),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR
                ),
                ..default()
            },
        }, RocketMissile,
        ));
    }
}


//szx яыч SZX