use bevy::prelude::*;
use bevy_aseprite::aseprite;

aseprite!(pub PresentAseprite, "sprites/present.aseprite");

#[derive(Component)]
pub struct Bullet {}
