mod plugins;

use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};
use plugins::{
    common::CommonPlugin, enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin,
    star::StarPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .add_plugins(EnemyPlugin)
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
