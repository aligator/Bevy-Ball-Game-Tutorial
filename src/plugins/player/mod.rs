use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_hit_star, enemy_hit_player));
    }
}
