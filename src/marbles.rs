// #![allow(clippy::unnecessary_cast)]

use std::collections::HashSet; // TODO: explore Bevy's hashset too (https://docs.rs/bevy/latest/bevy/utils/type.StableHashSet.html)

use crate::{fps::FpsPlugin, pause::PausePlugin};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
    utils::HashMap,
};
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct MarblesPlugin;
impl Plugin for MarblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PausePlugin, FpsPlugin))
            .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
            .insert_resource(SubstepCount(6))
            .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
            .add_systems(Startup, setup)
            .add_systems(Update, movement)
            // .add_systems(Update, print_collisions)
            .add_systems(FixedUpdate, merge_marbles);
    }
}

#[derive(Component)]
struct Marble;

#[derive(Component)]
struct MarbleConnections {
    connections2: HashMap<u32, u32>,
}

const MARBLE_RADIUS: f32 = 5.0;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Ceiling
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, 50.0 * 6.0, 0.0)
                .with_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, -50.0 * 6.0, 0.0)
                .with_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(-50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));

    let marble_mesh = meshes.add(shape::Circle::new(MARBLE_RADIUS).into());
    let marble_material_blue = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9)));
    let marble_material_purple = materials.add(ColorMaterial::from(Color::rgb(0.6, 0.2, 0.6)));
    let marble_material_green = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.9, 0.2)));

    let marble_scale = 10;
    let mut marble_entities: Vec<Entity> = Vec::new();
    let mut idx = 0;
    // Spawn stacks of marbles
    for x in -marble_scale..marble_scale {
        for y in -marble_scale..marble_scale {
            let marble = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: marble_mesh.clone().into(),
                    material: match idx {
                        0 => marble_material_purple.clone(),
                        1..=3 => marble_material_green.clone(),
                        _ => marble_material_blue.clone(),
                    },
                    transform: Transform::from_xyz(
                        x as f32 * 2.5 * MARBLE_RADIUS,
                        y as f32 * 2.5 * MARBLE_RADIUS,
                        0.0,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(MARBLE_RADIUS as Scalar),
                Marble {
                    // connections: vec![],
                },
                Name::new("marble"),
            ));
            marble_entities.push(marble.id());
            idx += 1;
        }
    }

    // let epsilon = 0.; // TODO: should we use?
    // let object1radius = marble_radius;
    // let object2radius = marble_radius;
    // let offset = object1radius + object2radius + epsilon;
    // commands.spawn(
    //     FixedJoint::new(marble_entities[0], marble_entities[2])
    //         .with_local_anchor_1(Vector::X * MARBLE_RADIUS) // collision normal (unit vector) times radius
    //         .with_local_anchor_2(-Vector::X * MARBLE_RADIUS), // collision normal (unit vector) times radius
    //                                                           // .with_rest_length(2.5 * MARBLE_RADIUS)
    //                                                           // .with_compliance(0.2),
    // );
    // commands.spawn(
    //     FixedJoint::new(marble_entities[1], marble_entities[2])
    //         .with_local_anchor_1(Vector::X * MARBLE_RADIUS) // collision normal (unit vector) times radius
    //         .with_local_anchor_2(-Vector::X * MARBLE_RADIUS), // collision normal (unit vector) times radius
    //                                                           // .with_rest_length(2.5 * MARBLE_RADIUS)
    //                                                           // .with_compliance(0.2),
    // );
    // commands.spawn(
    //     FixedJoint::new(marble_entities[2], marble_entities[3])
    //         .with_local_anchor_1(Vector::X * MARBLE_RADIUS) // collision normal (unit vector) times radius
    //         .with_local_anchor_2(-Vector::X * MARBLE_RADIUS), // collision normal (unit vector) times radius
    //                                                           // .with_rest_length(2.5 * MARBLE_RADIUS)
    //                                                           // .with_compliance(0.2),
    // );

    // TODO: Is this a good way to track this state?
    commands.spawn(MarbleConnections {
        connections2: HashMap::new(),
    });
}

// fn spawn_joint(mut commands: Commands, e1: Entity, e2: Entity, midpoint: Vec2) {
//     commands.spawn(
//         FixedJoint::new(e1, e2)
//             .with_local_anchor_1(midpoint) // collision normal (unit vector) times radius
//             .with_local_anchor_2(Vector::X) // collision normal (unit vector) times radius
//             // .with_rest_length(2.5 * MARBLE_RADIUS)
//             .with_compliance(0.2),
//     );
// }

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut marbles: Query<&mut LinearVelocity, With<Marble>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for mut linear_velocity in &mut marbles {
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            // Use a higher acceleration for upwards movement to overcome gravity
            linear_velocity.y += 2500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            linear_velocity.y -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            linear_velocity.x -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            linear_velocity.x += 500.0 * delta_time;
        }
    }
}

// fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
//     for Collision(contacts) in collision_event_reader.read() {
//         println!(
//             "Entities {:?} and {:?} are colliding",
//             contacts.entity1, contacts.entity2,
//         );
//     }
// }

// let merge_table: HashMap<(usize, usize), bool> = HashMap::new();

// mut merges = HashSet::new();

// TODO: Should we do this during the physics timestamp instead, if it depends on Collision events and mutates the physics world?
fn merge_marbles(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    mut marble_connections: Query<&mut MarbleConnections>,
    mut query: Query<(Entity, &Transform), With<Marble>>,
) {
    // TODO: reactive
    // return;

    let mut connection = marble_connections.single_mut();

    let mut marble_entities: HashMap<u32, &Transform> = HashMap::new();
    for (entity, transform) in query.iter() {
        marble_entities.insert(entity.index(), transform);
    }

    for Collision(contacts) in collision_event_reader.read() {
        let id1 = contacts.entity1.index();
        let id2 = contacts.entity2.index();
        // insert with lower ID pointing to higher ID
        let (lower, upper) = if id1 < id2 { (id1, id2) } else { (id2, id1) };

        if let Some(val) = connection.connections2.get(&lower) {
            if *val == upper {
                return;
            }
        }

        // let epsilon = 0.; // TODO: should we use?
        // let object1radius = marble_radius;
        // let object2radius = marble_radius;
        // let offset = object1radius + object2radius + epsilon;

        if let Some(e1) = marble_entities.get(&id1) {
            if let Some(e2) = marble_entities.get(&id2) {
                connection.connections2.insert(lower, upper);

                let midpoint = (e1.translation + e2.translation) / 2.;
                commands.spawn(
                    FixedJoint::new(contacts.entity1, contacts.entity2).with_compliance(0.005), // this seems necessary to have a stable physics simulation. Otherwise things go FLYING off screen
                );

                println!(
                    "Merging {:?} ({:?}) and {:?} ({:?})",
                    contacts.entity1, id1, contacts.entity2, id2
                );
            }
        }

        // TODO: Update the color of entity2
        // commands.get_entity(entity2);
        // contacts.entity2.get

        // break; // TODO: for debugging exploration.. merge at most one per timestep (concern: how does Physics timestep align with commands running?)
    }
}
