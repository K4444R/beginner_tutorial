use::std::ops::Range;
use std::thread::spawn;
use bevy::prelude::*;
use rand::Rng;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

//szx яыч SZX
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALE: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;

const RADIUS: f32 = 2.5;
#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer{ //структура данных Таймер, проверяющий, закончился ли процесс либо нет
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
            .add_systems(Update, (spawn_asteroids, rotate_asteroids, handle_asteroid_collisions));
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished(){
        return;
    }
    //Нужно подключить отдельный крейт для рандомных чисел
    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALE;

    commands.spawn((
        MovingObjectBundle {
        velocity: Velocity::new(velocity),
        acceleration: Acceleration::new(acceleration),
        collider: Collider::new(RADIUS),
        model: SceneBundle {
            scene: scene_assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    }, Asteroid, ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut(){
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
){
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter(){
            // Столкновение Астеройда с другим астероидом
            if query.get(collided_entity).is_ok() {
                continue; //яыч если существо сталкивается с себе подобным существом и находится внутри запроса 'query', то срабатывается условие
                //астероиды пока что будут проелетать между собой без сталкиваний, похже исправлю ^_^
            }


            // Деспавн астеройда (если астеройд столкнется с чем нибудь другим кроме другого астероида, просто уничтожится)
            // позже добавлю здоровье ^_^
            commands.entity(entity).despawn_recursive();  //если бы использовали обычный despawn(), исчез бы родительский элемент,
            // но не все дочерние элементы. А нужно задеспавнить одновременно,
            // поэтому используем despawn_recursive()


        }
    }
}