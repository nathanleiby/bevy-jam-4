use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::{math::*, prelude::*};

use crate::GameState;

// TODO: make this sort of param configurable from EGUI .. can I add some flags? or does it need to existing in Bevy land (like a Resource)?
const SPEED: f32 = 150.;
const RADIUS_MIN: f32 = 10.;
const RADIUS_MAX: f32 = 50.;
const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 800.;

pub struct GooPlugin;
impl Plugin for GooPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_goo.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_goo.run_if(in_state(GameState::Playing)))
            .add_systems(Update, despawn.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

// TODO: Can I put EGUI name on the Marker component instead of below?
#[derive(Component)]
struct Goo {
    created_at: f64,
}

fn spawn_goo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());

    if !spawn_timer.0.just_finished() {
        return;
    }

    // let x = rand::random::<f32>() * SCREEN_WIDTH;
    let x = 0.; // Debugging accretion
    let y = 0.; // SCREEN_HEIGHT / 2.;
    let radius = rand::random::<f32>() * (RADIUS_MAX - RADIUS_MIN) + RADIUS_MIN;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        },
        // physics
        RigidBody::Dynamic,
        Collider::ball(radius as Scalar),
        // marker
        Goo {
            created_at: time.elapsed_seconds_f64(),
        },
        // egui name
        Name::new("Goo"),
    ));
}

fn move_goo(mut goo_query: Query<&mut Transform, With<Goo>>, time: Res<Time>) {
    for mut goo_transform in &mut goo_query {
        goo_transform.translation.y -= SPEED * time.delta_seconds();
    }
}

fn despawn(mut commands: Commands, goo_query: Query<(Entity, &Transform, &Goo)>, time: Res<Time>) {
    let now = time.elapsed_seconds_f64();

    // despawn if older than 5 seconds
    for (entity, _, goo) in goo_query.iter() {
        if now - goo.created_at > 5. {
            commands.entity(entity).despawn_recursive();
        }
    }

    // despawn if goo moves off the screen (or outside arena due to physics bug)
    // for (entity, transform, _) in goo_query.iter() {
    //     if (transform.translation.y > SCREEN_HEIGHT ) < 0. {
    //         commands.entity(entity).despawn_recursive();
    //     }
    // }
}

// fn cleanup() {
//     todo!("despawn all goo if exiting Gameplay");
// }
