use crate::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

pub struct ScorePlugin;

/// Keeps track of the score and draws the Scoreboard.
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<ScoreValue>(ScoreValue(0))
            .register_type::<ScoreValue>() // for bevy-inspector-egui
            .add_event::<ScoreEvent>()
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

#[derive(Event, Debug)]
pub struct ScoreEvent(pub usize); // points

#[derive(Component)]
struct Scoreboard;

fn setup(mut commands: Commands) {
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
}

fn update_scoreboard_text(
    mut score_event_reader: EventReader<ScoreEvent>,
    mut score: ResMut<ScoreValue>,
    mut query: Query<&mut Text, With<Scoreboard>>,
) {
    let mut text = query.single_mut();
    for ev in score_event_reader.read() {
        score.0 += ev.0;
        let val = score.0;
        text.sections[1].value = format!("{val:>4.0}");
    }
}
