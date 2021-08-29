use rand::random;
use crate::phys::Phys;
use quicksilver::geom::Vector;
use std::collections::VecDeque;

pub struct EnemyFactory {
   m_difficulty: f32,
   m_positions: VecDeque<u64>
}

impl EnemyFactory {
   pub fn new() -> EnemyFactory{
      EnemyFactory {
         m_difficulty: 0.0,
         m_positions: VecDeque::new()
      }
   }

   pub fn tick(&mut self, phys: &mut Phys) {
      self.m_difficulty = 1.0 - (1.0 - self.m_difficulty) * 0.9999975;

      if random::<f32>() < self.m_difficulty + 0.005 {
         //create new enemy
         let enemy = phys
            .create_body()
            .set_pos(Vector::new(368.0, 10.0)) //x coord needs to be random
            .set_vel(Vector::new(0.0, 40.0))
            .set_radius(64.0)
            .set_mass(1.0)
            .id;

            self.m_positions.push_back(enemy);
      }
   }

   pub fn positions(&self) -> VecDeque<u64> {
      self.m_positions.clone()
   }
}