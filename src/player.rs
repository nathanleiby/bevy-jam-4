use crate::actions::Actions;

use crate::marbles::Marble;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::components::{AngularVelocity, Collider, LinearVelocity, RigidBody};
use bevy_xpbd_2d::math::{AdjustPrecision, Scalar};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(Update, rotate_player.run_if(in_state(GameState::Playing)));
    }
}

const PLAYER_RADIUS: f32 = 10.0;

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(shape::Circle::new(PLAYER_RADIUS).into());
    let material = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.9, 0.9)));

    let mesh2 = meshes.add(shape::Circle::new(PLAYER_RADIUS + 5.).into());
    let material2 = materials.add(ColorMaterial::from(Color::rgb(0., 0.8, 0.8)));

    let spawn_position = Vec3::new(-350., 0., 0.);

    let marble = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: material.clone(),
                transform: Transform::from_translation(spawn_position),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(PLAYER_RADIUS as Scalar),
            Marble {
                is_player_controlled: true,
            },
            Name::new("player"),
        ))
        .insert(Player)
        .with_children(|parent| {
            // // TODO: Give it a better glow effect at some point. Emphasize the player. Right now it's flashing due to z-index fighting
            // parent.spawn(MaterialMesh2dBundle {
            //     mesh: mesh2.clone().into(),
            //     material: material2.clone(),
            //     transform: Transform::from_xyz(0., 0., 0.0),
            //     ..default()
            // });

            // clarify which direction the player is facing
            let square_sprite = Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            };

            //
            parent.spawn((
                Name::new("player:direction_indicator"),
                SpriteBundle {
                    sprite: square_sprite.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_scale(Vec3::new(0.1, 0.5, 1.0)),
                    ..default()
                },
            ));
        });
}

const SPEED: f32 = 2000.0; // 500 to 2500
const ROTATION_SPEED: f32 = 500.;

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&Marble, &mut LinearVelocity, &mut AngularVelocity), With<Marble>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();
    let delta_x = actions.player_movement.unwrap().x * SPEED * delta_time;
    let delta_y = actions.player_movement.unwrap().y * SPEED * delta_time;
    for (marble, mut linear_velocity, mut angular_velocity) in query.iter_mut() {
        if !marble.is_player_controlled {
            continue;
        }
        linear_velocity.x += delta_x;
        linear_velocity.y += delta_y;
    }
}

fn rotate_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&Marble, &mut AngularVelocity), With<Marble>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();
    let delta_rotation = actions.player_rotation * ROTATION_SPEED * delta_time;
    for (marble, mut angular_velocity) in query.iter_mut() {
        if !marble.is_player_controlled {
            continue;
        }
        angular_velocity.0 = -delta_rotation;
    }
}
