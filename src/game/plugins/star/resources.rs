use bevy::prelude::*;

use super::STAR_SPAWN_INTERVAL;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
