use bevy::prelude::*;

use crate::plugins::common::events::GameOver;

use super::resources::{HighScores, Score};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for game_over in game_over_event_reader.iter() {
        println!("Game over! Score: {}", game_over.score);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for game_over in game_over_event_reader.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), game_over.score));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1));
        high_scores.scores.truncate(5);
        println!("New high scores: {:?}", high_scores.scores);
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores.scores);
    }
}
