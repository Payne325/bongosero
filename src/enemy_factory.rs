use rand::Rng;
use crate::phys::Phys;
use quicksilver::geom::Vector;
use std::collections::VecDeque;

pub struct EnemyFactory {
   m_difficulty: f32,
   m_refs: VecDeque<u64>
}

impl EnemyFactory {
   pub fn new() -> EnemyFactory{
      EnemyFactory {
         m_difficulty: 0.0,
         m_refs: VecDeque::new()
      }
   }

   pub fn tick(&mut self, phys: &mut Phys) {
      self.m_difficulty = 1.0 - (1.0 - self.m_difficulty) * 0.9999975;
      
      let mut rng = rand::thread_rng();
      
      if rng.gen::<f32>() < self.m_difficulty + 0.005 {
         //create new enemy
         let x = rng.gen_range(32.0, 768.0);
         let enemy = phys
            .create_body()
            .set_pos(Vector::new(x, 10.0)) //x coord needs to be random
            .set_vel(Vector::new(0.0, 40.0))
            .set_radius(64.0)
            .set_mass(1.0)
            .id;

            self.m_refs.push_back(enemy);
      }
   }

   pub fn existing_enemy_ids(&self) -> VecDeque<u64> {
      self.m_refs.clone()
   }

   pub fn remove(&mut self, index: usize) {
      self.m_refs.remove(index);
   }
}