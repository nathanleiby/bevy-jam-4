#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod config;
mod construction;
mod destruction;
mod fps;
mod goo;
mod level;
mod loading;
mod marbles;
mod menu;
mod pause;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;

use bevy::prelude::*;
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use construction::ConstructionPlugin;
use destruction::DestructionPlugin;
use goo::GooPlugin;
use level::LevelPlugin;
use marbles::MarblesPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            PhysicsPlugins::default(),
            LevelPlugin,
            GooPlugin,
            MarblesPlugin,
            ConstructionPlugin,
            DestructionPlugin,
        ));

        // TODO: re-enable for moar debuggin
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        // }

        app.add_plugins(PhysicsDebugPlugin::default());
    }
}
