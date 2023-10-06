use bevy::prelude::*;

use crate::AppState;

use self::{resources::HighScores, systems::*};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score)
            .init_resource::<HighScores>()
            .add_systems(
                Update,
                (
                    update_score.run_if(in_state(AppState::Game)),
                    update_high_scores,
                    high_scores_updated,
                ),
            )
            .add_systems(PostUpdate, handle_game_over);
    }
}
