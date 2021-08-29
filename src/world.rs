use crate::phys::Phys;
use crate::input_device::UserCommand;
use crate::enemy_factory::EnemyFactory;
use quicksilver::geom::Vector;
use std::collections::VecDeque;

pub struct World {
   m_phys: Phys,
   m_player: u64,
   m_bullets: VecDeque<u64>,
   m_enemy_factory: EnemyFactory
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
            m_bullets: VecDeque::new(),
            m_enemy_factory: EnemyFactory::new()
         }
   }

   pub fn maintain(&mut self, command: UserCommand) {
      {
         let mut player = self.m_phys.get_body_mut(self.m_player).unwrap();

         if command.m_pos != Vector::ZERO {
            player.set_pos(command.m_pos);
         }
         else {
            player.set_vel(command.m_move_dir);
         }
      }

      self.m_phys.tick(1.0/60.0); // estimating 60 fps

      self.m_enemy_factory.tick(&mut self.m_phys);

      let bullet_speed = 1000.0;

      if command.m_fire_bullet {
         let player_pos = self.get_player_position();

         self.m_bullets.push_back(self.m_phys
                                    .create_body()
                                    .set_pos(player_pos + Vector::new(16.0, -16.0)) //x coord = player.x + bullet height
                                    .set_vel(Vector::new(0.0, -bullet_speed))
                                    .set_radius(16.0)
                                    .set_mass(1.0)
                                    .id);
      }
   }
   
   pub fn get_player_position(&mut self) -> Vector {
      let mut pos : Vector;

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

   pub fn bullets(&self) -> VecDeque<Vector> {
      let mut positions: VecDeque<Vector> = VecDeque::new();

      for bullet_id in &self.m_bullets {
         let body_optional = self.m_phys.get_body(*bullet_id);

         match body_optional {
            Some(b) => positions.push_back(b.pos),
            None => {}
         }         
      }

      positions
   }

   pub fn enemies(&self) -> VecDeque<Vector> {
      let mut positions: VecDeque<Vector> = VecDeque::new();

      for bullet_id in &self.m_enemy_factory.positions() {
         let body_optional = self.m_phys.get_body(*bullet_id);

         match body_optional {
            Some(b) => positions.push_back(b.pos),
            None => {}
         }         
      }

      positions
   }
}