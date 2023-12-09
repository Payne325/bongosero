use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite::AsepriteBundle;
use bevy_aseprite::anim::AsepriteAnimation;

use super::components::FaceTracker;
use super::components::Player;

use crate::events::GameOver;
use crate::game::bullet::components::Bullet;
use crate::game::bullet::BULLET_SIZE;
use crate::game::bullet::components::PresentAseprite;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::resources::EnemySpawnTrigger;
use crate::game::enemy::*;
use crate::game::score::resources::*;
use crate::resources::Bongo;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const PLAYER_SPAWN_HEIGHT_REL: f32 = PLAYER_SIZE / 600.0;

const F_TRAK_MAX_BNDS: (i32, i32) = (600, 420); //Todo. Create some sort of calibration routine to get this scale
                                                //const MOVE_DEAD_ZONE: f32 = 0.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() * PLAYER_SPAWN_HEIGHT_REL,
                0.0,
            ),
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    face_capture: NonSend<FaceTracker>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        if let Ok(bbox) = face_capture.bbox_receiver.try_recv() {
            let x_pos = f_trak_to_screen_coords((bbox.0 .0 + bbox.1 .0) as f32 / 2.0, window);
            transform.translation.x = x_pos;
        } else {
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn player_fires_gun(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut bongo: ResMut<Bongo>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(transform) = player_query.get_single() {
        if keyboard_input.just_released(KeyCode::L) || bongo.check_gun_fired() {
            let position = transform.translation + Vec3::new(0.0, BULLET_SIZE / 2.0, 0.0);

            commands.spawn((
                AsepriteBundle {
                    aseprite: asset_server.load(PresentAseprite::PATH),
                    animation: AsepriteAnimation::from(PresentAseprite::tags::SPIN),
                    transform: Transform {
                        translation: position,
                        scale: Vec3::splat(1.),
                        ..default()
                    },
                    ..Default::default()
                },
                Bullet {},
            ));
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
    mut enemy_factory: ResMut<EnemySpawnTrigger>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
                enemy_factory.reset();
            }
        }
    }
}

pub fn setup_face_capture(world: &mut World) {
    world.insert_non_send_resource(FaceTracker::default());
}

pub fn ignore_face_track_frames(face_capture: NonSend<FaceTracker>) {
    // continue to read frames from face capture while game paused.
    // this should stabilise the movement at game start
    let _ = face_capture.bbox_receiver.try_recv();
    //println!("MARCOPOLO!");
}

fn f_trak_to_screen_coords(position: f32, window: &Window) -> f32 {
    let min = 0.0;
    let max = (F_TRAK_MAX_BNDS.0) as f32;
    let norm_pos = (position - min) / (max - min);

    //multiply to move to new coord system
    // also reverse so that player and char move in same direction
    window.width() - (norm_pos * window.width())
}
