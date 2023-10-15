use bevy::prelude::*;

use systems::interactions::*;
use systems::layout::*;

use crate::AppState;

mod components;
mod systems;
mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, main_menu)
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, (
                animate_buttons,
                interact_with_play_button,
                interact_with_quit_button,
            ).run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub fn main_menu() {
    println!("main menu");
}
