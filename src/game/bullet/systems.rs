use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Bullet;
use super::{BULLET_SIZE, BULLET_SPEED};

pub fn despawn_bullets(mut commands: Commands, bullet_query: Query<Entity, With<Bullet>>) {
    for bullet_entity in bullet_query.iter() {
        commands.entity(bullet_entity).despawn();
    }
}

pub fn bullet_movement(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in bullet_query.iter_mut() {
        transform.translation += Vec3::Y * BULLET_SPEED * time.delta_seconds();
    }
}

pub fn check_bullet_exceeded_bounds(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_bullet_size = BULLET_SIZE / 2.0;
    let y_max = window.height() - half_bullet_size;

    for (entity, mut transform) in bullet_query.iter_mut() {
        let translation = transform.translation;

        // If the enemy reaches the bottom of the screen, GAME OVER
        if translation.y > y_max {
            println!("Bullet out of bounds!");
            commands.entity(entity).despawn();
        }

        transform.translation = translation;
    }
}
