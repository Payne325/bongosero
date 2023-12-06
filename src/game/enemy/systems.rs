use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::events::GameOver;
use crate::game::bullet::components::Bullet;
use crate::game::bullet::BULLET_SIZE;
use crate::game::score::resources::Score;

// use crate::enemy::components::*;
use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPAWN_HEIGHT_REL, ENEMY_SPEED};

pub fn spawn_intial_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    spawn_single_enemy(&mut commands, &window, &asset_server);
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn check_enemy_reached_ground(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform), With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let y_min = 0.0 + half_enemy_size;

    for (entity, mut transform) in enemy_query.iter_mut() {
        let translation = transform.translation;

        // If the enemy reaches the bottom of the screen, GAME OVER
        if translation.y < y_min {
            println!("Enemy reached ground! Game Over!");
            let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
            audio.play(sound_effect);
            commands.entity(entity).despawn();
            game_over_event_writer.send(GameOver { score: score.value });
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            let bullet_radius = BULLET_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < bullet_radius + enemy_radius {
                println!("Enemy hit bullet! It dead!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

pub fn tick_enemy_spawn_trigger(
    mut enemy_spawn_trigger: ResMut<EnemySpawnTrigger>,
    time: Res<Time>,
) {
    enemy_spawn_trigger.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_trigger: ResMut<EnemySpawnTrigger>,
) {
    if enemy_spawn_trigger.should_spawn_enemy() {
        let window = window_query.get_single().unwrap();
        spawn_single_enemy(&mut commands, &window, &asset_server);
    }
}

fn spawn_single_enemy(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let x = random::<f32>() * window.width();
    let y = ENEMY_SPAWN_HEIGHT_REL * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("sprites/enemy.png"),
            ..default()
        },
        Enemy {
            direction: Vec2::new(0.0, -1.0).normalize(),
        },
    ));
}
