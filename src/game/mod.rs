use bevy::prelude::*;

mod plugins;
mod systems;

use crate::AppState;

use self::{
    plugins::{
        common::CommonPlugin, enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin,
        star::StarPlugin,
    },
    systems::{toggle_simulation, transition_to_game_state, transition_to_menu_state},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // .add_systems(schedule, systems)
            .add_plugins(CommonPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(EnemyPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(Update, (transition_to_menu_state, transition_to_game_state));
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum SimulationState {
    Running,

    #[default]
    Paused,
}
