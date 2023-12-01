use rand::Rng;
use crate::phys::Phys;
use quicksilver::geom::Vector;
use std::collections::VecDeque;

const MIN_ENEMY_SPEED : f32 = 35.0;
pub struct EnemyFactory {
   m_difficulty: i32,
   m_timer: f32,
   m_refs: VecDeque<u64>
}

impl EnemyFactory {
   pub fn new() -> EnemyFactory{
      EnemyFactory {
         m_difficulty: 1,
         m_timer: 0.0,
         m_refs: VecDeque::new()
      }
   }

   pub fn tick(&mut self, phys: &mut Phys, tick: f32) {
      self.m_timer += tick;
      
      if self.m_difficulty != 10 && self.m_timer > 30.0 {

         self.m_timer = 0.0;
         self.m_difficulty += 1;
      } 

      let mut rng = rand::thread_rng();
      
      if rng.gen_range(0..100) < self.m_difficulty {
         //create new enemy

         //range = (half enemy sprite + min offset , screen width - half enemy sprite - min offset)
         let x_pos = rng.gen_range(57.0..743.0);
         let y_vel = MIN_ENEMY_SPEED * self.m_difficulty as f32;

         if cfg!(feature = "debug") {
            println!("ENEMY_FACTORY: {{ LEVEL: {}, TIMER: {} }}", self.m_difficulty, self.m_timer);
         }

         let enemy = phys
            .create_body()
            .set_pos(Vector::new(x_pos, 10.0))
            .set_vel(Vector::new(0.0, y_vel))
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