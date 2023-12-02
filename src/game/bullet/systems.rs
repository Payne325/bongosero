use bevy::prelude::*;

use super::BULLET_SPEED;
use super::components::Bullet;

pub fn despawn_bullets(mut commands: Commands, bullet_query: Query<Entity, With<Bullet>>) {
    for bullet_entity in bullet_query.iter() {
        commands.entity(bullet_entity).despawn();
    }
}

pub fn bullet_movement(
    mut bullet_query: Query<&mut Transform, With<Bullet>>,
    time: Res<Time>,
){
    for mut transform in bullet_query.iter_mut() {
        transform.translation += Vec3::Y * BULLET_SPEED * time.delta_seconds();
    }
}
