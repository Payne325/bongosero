use bevy::prelude::*;
use std::time::Duration;

pub const DIFFICULTY_CHANGE_TIME: f32 = 30.0;
const ENEMY_SPAWN_COOLDOWN: f32 = 5.0;

#[derive(Resource)]
pub struct EnemySpawnTrigger {
    spawn_cooldown_timer: Timer,
    difficulty_timer: Timer,
    difficulty: u8,
}

impl Default for EnemySpawnTrigger {
    fn default() -> EnemySpawnTrigger {
        EnemySpawnTrigger {
            spawn_cooldown_timer: Timer::from_seconds(ENEMY_SPAWN_COOLDOWN, TimerMode::Once),
            difficulty_timer: Timer::from_seconds(DIFFICULTY_CHANGE_TIME, TimerMode::Once),
            difficulty: 1,
        }
    }
}

impl EnemySpawnTrigger {
    pub fn tick(&mut self, dt: Duration) {
        if self.difficulty_timer.paused() {
            self.difficulty_timer.unpause();
        }

        self.difficulty_timer.tick(dt);
        self.spawn_cooldown_timer.tick(dt);

        if self.difficulty != 10 && self.difficulty_timer.finished() {
            self.difficulty_timer.reset();
            self.difficulty += 1;
        }
    }

    pub fn should_spawn_enemy(&mut self) -> bool {
        if self.spawn_cooldown_timer.finished() {
            self.spawn_cooldown_timer
                .set_duration(Duration::from_secs_f32(
                    ENEMY_SPAWN_COOLDOWN / self.difficulty as f32,
                ));
            self.spawn_cooldown_timer.reset();
            return true;
        }

        false
    }

    pub fn reset(&mut self) {
        self.difficulty = 1;
        self.difficulty_timer.pause();
        self.difficulty_timer.reset();
    }
}
