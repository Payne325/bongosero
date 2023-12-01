use crate::enemy_factory::EnemyFactory;
use crate::input_device::UserCommand;
use crate::phys::{Body, Phys};
use quicksilver::geom::Vector;
use std::collections::VecDeque;

pub struct World {
    m_phys: Phys,
    m_player: u64,
    m_existing_bullets: VecDeque<u64>,
    m_enemy_factory: EnemyFactory,
    m_game_over: bool,
}

impl World {
    pub fn new() -> Self {
        let mut phys = Phys::new().with_gravity(Vector::new(0.0, 16.0));

        let player = phys
            .create_body()
            .set_pos(Vector::new(368.0, 516.0)) //x coord = (screen width/2) - (sprite width/2)
            .set_vel(Vector::new(0.0, 0.0))
            .set_radius(64.0)
            .set_mass(1.0)
            .set_as_player()
            .id;

        Self {
            m_phys: phys,
            m_player: player,
            m_existing_bullets: VecDeque::new(),
            m_enemy_factory: EnemyFactory::new(),
            m_game_over: false,
        }
    }

    pub fn maintain(&mut self, command: UserCommand) {
        if self.m_game_over {
            return;
        }

        {
            let mut player = self.m_phys.get_body_mut(self.m_player).unwrap();

            if command.m_pos != Vector::ZERO {
                player.set_pos(command.m_pos);
            } else {
                player.set_vel(command.m_move_dir);
            }
        }

        let tick = 1.0 / 60.0; // estimated delta assuming 60 fps
        self.m_phys.tick(tick);

        //remove out of bounds items
        self.handle_bullets_out_of_bounds();
        self.handle_enemies_out_of_bounds();

        // Spawn new physics bodies
        self.m_enemy_factory.tick(&mut self.m_phys, tick);

        let bullet_speed = 1000.0;

        if command.m_fire_bullet {
            let player_pos = self.get_player_position();

            self.m_existing_bullets.push_back(
                self.m_phys
                    .create_body()
                    .set_pos(player_pos + Vector::new(16.0, -16.0)) //x coord = player.x + bullet height
                    .set_vel(Vector::new(0.0, -bullet_speed))
                    .set_radius(16.0)
                    .set_mass(1.0)
                    .id,
            );
        }

        //handle any collisions
        self.collision_detection();

        self.clean_up_HACK();
    }

    pub fn get_player_position(&mut self) -> Vector {
        let mut pos: Vector;

        {
            let mut player = self.m_phys.get_body_mut(self.m_player).unwrap();
            pos = player.pos;

            //Keep player position bound to game window
            if pos.x < 0.0 {
                pos.x = 0.0;
            }

            if pos.x > 784.0 {
                pos.x = 784.0;
            }

            player.set_pos(pos);
        }
        pos
    }

    pub fn bullet_positions(&self) -> VecDeque<Vector> {
        let mut positions: VecDeque<Vector> = VecDeque::new();

        for bullet_id in &self.m_existing_bullets {
            if let Some(b) = self.m_phys.get_body(*bullet_id) {
                positions.push_back(b.pos)
            }
        }

        positions
    }

    pub fn enemy_positions(&self) -> VecDeque<Vector> {
        let mut positions: VecDeque<Vector> = VecDeque::new();

        for enemy_id in &self.m_enemy_factory.existing_enemy_ids() {
            if let Some(b) = self.m_phys.get_body(*enemy_id) {
                positions.push_back(b.pos)
            }
        }

        positions
    }

    pub fn game_over(&self) -> bool {
        self.m_game_over
    }

    fn collision_detection(&mut self) {
        //hard coded sprite sizes
        //bullets 32x32
        //enemies 64x64
        //player 64x64

        let mut to_remove: VecDeque<u64> = VecDeque::new();
        let mut handle_enemy_other_collision = |enemy_id: u64,
                                                enemy: &std::cell::Ref<Body>,
                                                other_id: u64,
                                                other_optional: Option<std::cell::Ref<Body>>,
                                                detection_range: f32|
         -> bool {
            match other_optional {
                Some(other) => {
                    let distance = (other.pos.x - enemy.pos.x).powf(2.0)
                        + (other.pos.y - enemy.pos.y).powf(2.0);
                    if distance.sqrt() < detection_range {
                        to_remove.push_back(enemy_id);
                        to_remove.push_back(other_id);
                        return true;
                    }

                    false
                }
                None => false,
            }
        };

        // check for collisions between enemies and other bodies
        for enemy_id in &self.m_enemy_factory.existing_enemy_ids() {
            let enemy_optional = self.m_phys.get_body(*enemy_id);

            if let Some(enemy) = enemy_optional {
                for bullet_id in &self.m_existing_bullets {
                    let detection_range = 48.0; // half bullet size + half enemy size
                    let _ = handle_enemy_other_collision(
                        *enemy_id,
                        &enemy,
                        *bullet_id,
                        self.m_phys.get_body(*bullet_id),
                        detection_range,
                    );
                }

                let detection_range = 64.0; // half player size + half enemy size
                let player_hit = handle_enemy_other_collision(
                    *enemy_id,
                    &enemy,
                    self.m_player,
                    self.m_phys.get_body(self.m_player),
                    detection_range,
                );
                if player_hit {
                    self.m_game_over = true;
                }
            }
        }

        for id in to_remove {
            self.m_phys.delete_body(id);
        }
    }

    fn handle_bullets_out_of_bounds(&mut self) {
        let mut to_remove: VecDeque<u64> = VecDeque::new();

        for bullet_id in &self.m_existing_bullets {
            if let Some(b) = self.m_phys.get_body(*bullet_id) {
                if b.pos.y < 16.0 {
                    to_remove.push_back(*bullet_id);
                }
            }
        }

        for id in to_remove {
            self.m_phys.delete_body(id);
        }
    }

    fn handle_enemies_out_of_bounds(&mut self) {
        let mut to_remove: VecDeque<u64> = VecDeque::new();

        for enemy_id in &self.m_enemy_factory.existing_enemy_ids() {
            if let Some(b) = self.m_phys.get_body(*enemy_id) {
                if b.pos.y > 532.0 {
                    to_remove.push_back(*enemy_id);

                    //Enemy reached the bottom, game ends
                    self.m_game_over = true;
                }
            }
        }

        for id in to_remove {
            self.m_phys.delete_body(id);
        }
    }

    #[allow(non_snake_case)]
    fn clean_up_HACK(&mut self) {
        //remove dangling references
        // nb: Enemy factory and world hold references to enemy and bullet bodies
        // collision detection and OOB checks remove the bodies but don't clear the reference
        // todo: refactor to ensure references are removed when bodies destroyed.

        let mut bullets_to_remove: VecDeque<usize> = VecDeque::new();

        for i in 0..self.m_existing_bullets.len() {
            let bullet_id = self.m_existing_bullets[i];
            let body = self.m_phys.get_body(bullet_id);

            if body.is_none() {
                bullets_to_remove.push_back(i);
            }
        }

        //Go in reverse order to preserve indices
        for b in bullets_to_remove.iter().rev() {
            self.m_existing_bullets.remove(*b);
        }

        let mut enemies_to_remove: VecDeque<usize> = VecDeque::new();
        let enemies = self.m_enemy_factory.existing_enemy_ids();
        for (i, enemy_id) in enemies.iter().enumerate() {
            let body = self.m_phys.get_body(*enemy_id);

            if body.is_none() {
                enemies_to_remove.push_back(i)
            }
        }

        //Go in reverse order to preserve indices
        for b in enemies_to_remove.iter().rev() {
            self.m_enemy_factory.remove(*b);
        }
    }
}
