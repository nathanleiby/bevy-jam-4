use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, bootstrap_level);
    }
}

#[derive(Component)]
pub struct Goal {}

fn setup(mut commands: Commands) {
    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Ceiling
    commands.spawn((
        Name::new("Wall:ceiling"),
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
        Name::new("Wall:floor"),
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
        Name::new("Wall:left"),
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
        Name::new("Wall:right"),
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));

    let goal_sprite = Sprite {
        color: Color::rgb(0.2, 0.02, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // right goal
    commands.spawn((
        Name::new("Goal:right"),
        SpriteBundle {
            sprite: goal_sprite.clone(),
            transform: Transform::from_xyz(50.0 * 9.5, 0.0, 0.0)
                .with_scale(Vec3::new(2.0, 2.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
        Goal {},
    ));
}

fn setup_level_one() {
    todo!("small ball")
    // RADIUS_MIN
}

fn setup_level_two() {
    todo!("big ball") // can push it but soooo slow to do so
                      // RADIUS_MAX * 2.
}

fn setup_level_three() {
    todo!("big ball and construction goo")
}

fn setup_level_four() {
    todo!("don't die! explain destruction goo")
}

// TODO: draw a grid on top of 2d UI, for debugging. e.g. so i can easily figure out 100px offsets
// fn debug_grid(
//     mut commands: Commands,
//     meshes: ResMut<Assets<Mesh>>,
//     materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let purple = materials.add(ColorMaterial::from(Color::PURPLE));

//     commands.spawn(MaterialMeshBundle {
//         mesh: meshes.add(Mesh::from(LineList {
//             lines: vec![
//                 (Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0)),
//                 (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
//             ],
//         })),
//         transform: Transform::from_xyz(-1.5, 0.0, 0.0),
//         material: purple.clone(),
//         ..default()
//     });
// }

fn bootstrap_level() {}
