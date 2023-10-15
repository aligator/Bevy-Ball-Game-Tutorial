use bevy::prelude::*;

use crate::AppState;
use crate::game::systems::{pause_simulation, resume_simulation};

use self::{
    plugins::{
        common::CommonPlugin, enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin,
        star::StarPlugin,
    },
    systems::{toggle_simulation, transition_to_game_state, transition_to_menu_state},
};

mod plugins;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins(CommonPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_plugins(EnemyPlugin)
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(Update, (
                toggle_simulation.run_if(in_state(AppState::Game)),
                transition_to_menu_state,
                transition_to_game_state
            ))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
