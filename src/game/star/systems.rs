use bevy::prelude::*;

use super::BULLET_SPEED;
use super::components::Star;

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn bullet_movement(
    mut bullet_query: Query<&mut Transform, With<Star>>,
    time: Res<Time>,
){
    for mut transform in bullet_query.iter_mut() {
        transform.translation += Vec3::Y * BULLET_SPEED * time.delta_seconds();
    }
}

