use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};
use bevy_framepace::FramepacePlugin;
use isomaze_lib::graphics::geometry;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "isomaze".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..Default::default()
        }))
        .add_plugins(FramepacePlugin)
        // .add_systems(StartUp, setup)
        // .add_systems(Update,(system, update_config))
        .run();
}

fn setup() {}
