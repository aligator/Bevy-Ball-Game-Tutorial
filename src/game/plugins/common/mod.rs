use bevy::prelude::*;

use self::{events::GameOver, systems::load_resources};

pub mod events;
pub mod resources;
mod systems;

pub fn confine_entity(position: Vec3, size: f32, window: &Window) -> Vec3 {
    let half_size = size / 2.0;
    let x_min = 0.0 + half_size;
    let x_max = window.width() - half_size;
    let y_min = 0.0 + half_size;
    let y_max = window.height() - half_size;

    Vec3 {
        x: num::clamp(position.x, x_min, x_max),
        y: num::clamp(position.y, y_min, y_max),
        z: 0.0,
    }
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_resources)
            .add_event::<GameOver>();
    }
}
