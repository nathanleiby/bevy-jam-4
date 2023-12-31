// #![allow(clippy::unnecessary_cast)]

use std::collections::HashSet; // TODO: explore Bevy's hashset too (https://docs.rs/bevy/latest/bevy/utils/type.StableHashSet.html)

use crate::{fps::FpsPlugin, pause::PausePlugin};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::{math::*, prelude::*};

// TODO: Merge this and ConstructionGoo into one thing?
pub struct MarblesPlugin;
impl Plugin for MarblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PausePlugin, FpsPlugin))
            .add_event::<MergeEvent>()
            // TODO: refactor core physics into own plugin (SubstepCount, Gravity)
            // .insert_resource(SubstepCount(6))
            .insert_resource(SubstepCount(12))
            .insert_resource(Gravity(Vector::ZERO))
            .add_systems(Startup, setup)
            // .add_systems(Update, movement)
            // .add_systems(Update, print_collisions)
            .add_systems(Update, merge)
            .add_systems(Update, handle_merge_events);
    }
}

#[derive(Component)]
pub struct Marble {
    pub is_player_controlled: bool,
}

#[derive(Component)]
struct MarbleConnections {
    // TODO: Should we use a HashSet instead?
    connections: HashSet<(u32, u32)>,
}

const MARBLE_RADIUS: f32 = 5.0;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let marble_mesh = meshes.add(shape::Circle::new(MARBLE_RADIUS).into());
    let marble_material_blue = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9)));
    // let marble_material_purple = materials.add(ColorMaterial::from(Color::rgb(0.6, 0.2, 0.6)));
    // let marble_material_green = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.9, 0.2)));

    let marble_scale = 3;
    let mut marble_entities: Vec<Entity> = Vec::new();
    let mut idx = 0;
    // Spawn stacks of marbles
    for x in -marble_scale..marble_scale {
        for y in -marble_scale..marble_scale {
            let marble = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: marble_mesh.clone().into(),
                    material: match idx {
                        // 0 => marble_material_purple.clone(),
                        // 1..=3 => marble_material_green.clone(),
                        _ => marble_material_blue.clone(),
                    },
                    transform: Transform::from_xyz(
                        x as f32 * 2.5 * MARBLE_RADIUS - 100.,
                        y as f32 * 2.5 * MARBLE_RADIUS,
                        0.0,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(MARBLE_RADIUS as Scalar),
                Marble {
                    is_player_controlled: false,
                },
                Name::new("marble"),
            ));
            marble_entities.push(marble.id());
            idx += 1;
        }
    }

    commands.spawn(MarbleConnections {
        connections: HashSet::new(),
    });
}

#[derive(Event, Debug)]
struct MergeEvent(Entity, Entity);

fn merge(
    mut collision_event_reader: EventReader<Collision>,
    mut marble_connections: Query<&mut MarbleConnections>,
    query: Query<(Entity, &Marble), With<Marble>>,
    mut merge_events: EventWriter<MergeEvent>,
) {
    let mut connection = marble_connections.single_mut();

    let mut playerControlledIDs: HashSet<u32> = HashSet::new();
    let mut marbleEntityIDs: HashSet<u32> = HashSet::new();
    for (entity, marble) in query.iter() {
        marbleEntityIDs.insert(entity.index());
        if marble.is_player_controlled {
            // println!("Player controlled marble: {:?}", entity.index());
            playerControlledIDs.insert(entity.index());
        }
    }

    // let mut marble_entities: HashMap<u32, Entity> = HashMap::new();
    for Collision(contacts) in collision_event_reader.read() {
        let id1 = contacts.entity1.index();
        let id2 = contacts.entity2.index();

        // check if id1 and id2 are both marbles
        if !marbleEntityIDs.contains(&id1) || !marbleEntityIDs.contains(&id2) {
            continue;
        }

        // check if at least oen is player controlled currently
        if !(playerControlledIDs.contains(&id1) || playerControlledIDs.contains(&id2)) {
            continue;
        }

        // insert with lower ID pointing to higher ID
        let (lower, upper) = if id1 < id2 { (id1, id2) } else { (id2, id1) };

        let newly_added = connection.connections.insert((lower, upper));
        if newly_added {
            merge_events.send(MergeEvent(contacts.entity1, contacts.entity2));
        } else {
        }
    }
}

fn handle_merge_events(
    mut merge_events: EventReader<MergeEvent>,
    mut commands: Commands,
    // mut marble_connections: Query<&mut MarbleConnections>,
    mut query: Query<(Entity, &mut Marble, &mut Handle<ColorMaterial>), With<Marble>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let marble_material_green = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.9, 0.2)));
    for ev in merge_events.read() {
        commands.spawn(
            FixedJoint::new(ev.0, ev.1).with_compliance(0.0001),
            // with_compliance seems necessary to have a stable physics simulation.
            // Otherwise things go FLYING off screen
        );

        // Look for a matching marble from the query
        for (entity, mut marble, mut material) in query.iter_mut() {
            if entity.index() == ev.0.index() || entity.index() == ev.1.index() {
                *material = marble_material_green.clone();
                marble.is_player_controlled = true;
            }
        }
    }
}
