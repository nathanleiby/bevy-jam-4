use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::HashSet};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use bevy_xpbd_2d::{math::*, prelude::*};

use crate::{level::Goal, marbles::Marble, score::ScoreEvent, GameState};

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
            .add_systems(Update, despawn.run_if(in_state(GameState::Playing)))
            .add_systems(Update, score.run_if(in_state(GameState::Playing)));
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

fn score(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    mut score_event_writer: EventWriter<ScoreEvent>,

    query_goo: Query<Entity, With<Goo>>,
    query_goal: Query<Entity, With<Goal>>,
) {
    let mut goo_entity_ids: HashSet<u32> = HashSet::new();
    for entity in query_goo.iter() {
        goo_entity_ids.insert(entity.index());
    }

    let mut goal_entity_id = 0;
    // assign query result to player_entity_id if it exists..
    if let Ok(result) = query_goal.get_single() {
        goal_entity_id = result.index();
    } else {
        return;
        // Goal not yet loaded
    }

    // let mut marble_entities: HashMap<u32, Entity> = HashMap::new();
    for Collision(contacts) in collision_event_reader.read() {
        let id1 = contacts.entity1.index();
        let id2 = contacts.entity2.index();

        // check if one is a goo and the other is the Goal
        if goo_entity_ids.contains(&id1) && id2 == goal_entity_id
            || goo_entity_ids.contains(&id2) && id1 == goal_entity_id
        {
            score_event_writer.send(ScoreEvent(1));
            if id1 == goal_entity_id {
                commands.entity(contacts.entity2).despawn_recursive();
            } else {
                commands.entity(contacts.entity1).despawn_recursive();
            }
        }
    }
}

fn move_goo(mut goo_query: Query<&mut Transform, With<Goo>>, time: Res<Time>) {
    for mut goo_transform in &mut goo_query {
        goo_transform.translation.y -= SPEED * time.delta_seconds();
    }
}

fn despawn(mut commands: Commands, goo_query: Query<(Entity, &Transform, &Goo)>, time: Res<Time>) {
    let now = time.elapsed_seconds_f64();

    // despawn if old enough
    for (entity, _, goo) in goo_query.iter() {
        if now - goo.created_at > 10. {
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
