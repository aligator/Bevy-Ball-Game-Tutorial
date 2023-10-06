use bevy::prelude::*;

use self::{
    resources::{HighScores, Score},
    systems::{handle_game_over, high_scores_updated, update_high_scores, update_score},
};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(
                Update,
                (update_score, update_high_scores, high_scores_updated),
            )
            .add_systems(PostUpdate, handle_game_over);
    }
}
