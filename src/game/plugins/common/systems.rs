use bevy::prelude::*;

use super::resources::SoundHandles;

pub fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundHandles {
        ditch: vec![
            asset_server.load("audio/pluck_001.ogg"),
            asset_server.load("audio/pluck_002.ogg"),
        ],
        collide: asset_server.load("audio/explosionCrunch_000.ogg"),
        collect: asset_server.load("audio/laserLarge_000.ogg"),
    });
}
