// #![allow(clippy::unnecessary_cast)]

use std::collections::HashSet;

use crate::marbles::Marble;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::{math::*, prelude::*};

pub struct ConstructionPlugin;
impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, collide_and_maybe_merge);
    }
}

#[derive(Component)]
struct ConstructionGoo {}

const CONSTRUCTION_GOO_RADIUS: f32 = 3.0;

// pub fn ConstructionGooBundle(
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) -> Bundle {
//     let marble_mesh = meshes.add(shape::Circle::new(CONSTRUCTION_GOO_RADIUS).into());
//     let material_red = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.1, 0.1)));

//     let marble_scale = 10;

//     return (
//         MaterialMesh2dBundle {
//             mesh: marble_mesh.clone().into(),
//             material: material_red.clone(),
//             transform: Transform::from_xyz(
//                 x as f32 * 2.5 * CONSTRUCTION_GOO_RADIUS,
//                 y as f32 * 2.5 * CONSTRUCTION_GOO_RADIUS + 100.,
//                 0.0,
//             ),
//             ..default()
//         },
//         RigidBody::Dynamic,
//         Collider::ball(CONSTRUCTION_GOO_RADIUS as Scalar),
//         ConstructionGoo {},
//         Name::new("Construction Goo"),
//     );
// }
fn setup(mut commands: Commands) {
    // // Spawn stacks of marbles
    // for x in -marble_scale..marble_scale {
    //     for y in -marble_scale..marble_scale {
    //         commands.spawn();
    //     }
    // }
}

fn collide_and_maybe_merge(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    query: Query<Entity, With<ConstructionGoo>>,
    query2: Query<Entity, With<Marble>>,
) {
    // marbles
    let mut marble_entity_ids: HashSet<u32> = HashSet::new();
    for entity in query2.iter() {
        marble_entity_ids.insert(entity.index());
    }

    // destruction goos
    let mut destruction_goo_entity_ids: HashSet<u32> = HashSet::new();
    for entity in query.iter() {
        destruction_goo_entity_ids.insert(entity.index());
    }

    // let mut marble_entities: HashMap<u32, Entity> = HashMap::new();
    for Collision(contacts) in collision_event_reader.read() {
        let id1 = contacts.entity1.index();
        let id2 = contacts.entity2.index();

        // check if one is a marble and the other is a destruction goo
        if (marble_entity_ids.contains(&id1) && destruction_goo_entity_ids.contains(&id2))
            || (marble_entity_ids.contains(&id2) && destruction_goo_entity_ids.contains(&id1))
        {
            // destroy both
            debug!("Destroying marble and destruction goo");
            commands.entity(contacts.entity1).despawn_recursive();
            commands.entity(contacts.entity2).despawn_recursive();
        }
    }
}
