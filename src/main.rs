mod game;
mod main_menu;

use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, close_on_esc)
        .run();
}
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
