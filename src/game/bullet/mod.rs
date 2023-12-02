use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;

pub const BULLET_SIZE: f32 = 32.0; // This is the bullet sprite size.
pub const BULLET_SPEED: f32 = 200.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_system(bullet_movement
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)))
            // On Exit State
            .add_system(despawn_bullets.in_schedule(OnExit(AppState::Game)));
    }
}
