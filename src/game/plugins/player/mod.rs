use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::{game::SimulationState, AppState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(
                Update,
                (player_movement, player_hit_star, enemy_hit_player)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
