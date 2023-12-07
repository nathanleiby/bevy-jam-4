use crate::actions::Actions;

use crate::marbles::Marble;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::components::{Collider, LinearVelocity, RigidBody};
use bevy_xpbd_2d::math::{AdjustPrecision, Scalar};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

const PLAYER_RADIUS: f32 = 25.0;

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh = meshes.add(shape::Circle::new(PLAYER_RADIUS).into());
    let material = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.9, 0.9)));
    let spawn_position = Vec2::new(-300., 0.);
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                material: material.clone(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(PLAYER_RADIUS as Scalar),
            Name::new("Player"),
        ))
        .insert(Marble {
            is_player_controlled: true,
        })
        .insert(Player);
}

const SPEED: f32 = 2000.0; // 500 to 2500

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut query: Query<(&Marble, &mut LinearVelocity), With<Marble>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();
    let delta_x = actions.player_movement.unwrap().x * SPEED * delta_time;
    let delta_y = actions.player_movement.unwrap().y * SPEED * delta_time;

    for (marble, mut linear_velocity) in query.iter_mut() {
        if !marble.is_player_controlled {
            continue;
        }
        linear_velocity.x += delta_x;
        linear_velocity.y += delta_y;
    }
}
