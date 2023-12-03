use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Default)]
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        // requires PhysicsPlugins::default()
        app.add_state::<AppState>()
            .add_systems(
                OnEnter(AppState::Paused),
                |mut time: ResMut<Time<Physics>>| time.pause(),
            )
            .add_systems(
                OnExit(AppState::Paused),
                |mut time: ResMut<Time<Physics>>| time.unpause(),
            )
            .add_systems(Update, pause_button)
            .add_systems(Update, step_button.run_if(in_state(AppState::Paused)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Paused,
    #[default]
    Running,
}

fn pause_button(
    current_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::P) {
        let new_state = match current_state.get() {
            AppState::Paused => AppState::Running,
            AppState::Running => AppState::Paused,
        };
        next_state.0 = Some(new_state);
    }
}

fn step_button(mut time: ResMut<Time<Physics>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Return) {
        time.advance_by(Duration::from_secs_f64(1.0 / 60.0));
    }
}
