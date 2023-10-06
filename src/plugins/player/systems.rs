use bevy::{prelude::*, window::PrimaryWindow};

use crate::plugins::{
    common::{
        confine_entity,
        events::GameOver,
        resources::{sound, SoundHandles},
    },
    enemy::{components::Enemy, ENEMY_SIZE},
    score::resources::Score,
    star::{components::Star, STAR_SIZE},
};

use super::components::Player;

const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        transform.translation = confine_entity(transform.translation, PLAYER_SIZE, window);
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    sounds: Res<SoundHandles>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game over!");
                commands.spawn(sound(sounds.collide.clone()));
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value })
            };
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    sounds: Res<SoundHandles>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                println!("Player hit star!");
                commands.spawn(sound(sounds.collect.clone()));
                commands.entity(star_entity).despawn();
                score.value += 1;
            };
        }
    }
}
