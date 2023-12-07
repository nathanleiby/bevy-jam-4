use crate::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

pub struct ScorePlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<ScoreValue>(ScoreValue(0))
            .register_type::<ScoreValue>() // for bevy-inspector-egui
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                update_scoreboard_text.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct ScoreValue(pub usize);

#[derive(Component)]
struct Scoreboard;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Scoreboard,
        Name::new("Scoreboard"),
        TextBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                ..default()
            },
            text: Text::from_sections([
                TextSection {
                    value: "Score: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                },
            ]),
            ..default()
        },
    ));
    // .with_children(|parent| {
    //     parent.spawn(TextBundle::from_section(
    //         "Score: 0",
    //         TextStyle {
    //             font_size: 15.0,
    //             color: Color::rgb(0.9, 0.9, 0.9),
    //             ..default()
    //         },
    //     ));
    // });
}

fn update_scoreboard_text(score: Res<ScoreValue>, mut query: Query<&mut Text, With<Scoreboard>>) {
    for mut text in &mut query {
        {
            let val = score.0;
            text.sections[1].value = format!("{val:>4.0}");
        }
    }
}
