use bevy::prelude::*;

#[derive(Resource)]
pub struct SoundHandles {
    pub ditch: Vec<Handle<AudioSource>>,
    pub collide: Handle<AudioSource>,
    pub collect: Handle<AudioSource>,
}

/// Returns a bundle for playing a sound one time.
/// Use with
/// ```
/// commands.spawn(sound(sounds.collide_sound.clone()));
/// ```
pub fn sound(handle: Handle<AudioSource>) -> AudioBundle {
    AudioBundle {
        source: handle,
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            ..default()
        },
    }
}
