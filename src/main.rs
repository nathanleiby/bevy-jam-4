// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        // .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // .insert_resource(Game::) // TODO: explore booting directly into GameState -- or otherwise writing code to simulate immediate click. need to make sure everything loaded 1st
        .add_plugins(GamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
