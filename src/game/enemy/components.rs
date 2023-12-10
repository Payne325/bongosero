use bevy::prelude::*;
use bevy_aseprite::aseprite;

aseprite!(pub EnemyAseprite, "sprites/enemy.aseprite");

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
