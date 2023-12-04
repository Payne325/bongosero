use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

pub const DIFFICULTY_CHANGE_TIME: f32 = 30.0;

#[derive(Resource)]
pub struct EnemySpawnTrigger {
    timer: Timer,
    difficulty: u8
}

impl Default for EnemySpawnTrigger {
    fn default() -> EnemySpawnTrigger {
        EnemySpawnTrigger {
            timer: Timer::from_seconds(DIFFICULTY_CHANGE_TIME, TimerMode::Repeating),
            difficulty: 1
        }
    }
}

impl EnemySpawnTrigger {
    pub fn tick(&mut self, dt: Duration) {
        self.timer.tick(dt);

        if self.difficulty != 10 && self.timer.finished() {
            self.difficulty += 1;
        }
    }

    pub fn should_spawn_enemy(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..100) < self.difficulty
    }
}
