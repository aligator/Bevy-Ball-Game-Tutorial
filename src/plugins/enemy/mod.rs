use bevy::prelude::*;

use self::systems::{
    enemy_movement, spawn_enemies, spawn_enemy_over_time, tick_enemy_spawn_timer,
    update_enemy_direction,
};

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemy_over_time,
                ),
            );
    }
}
