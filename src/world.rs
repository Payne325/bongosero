use crate::phys::{Phys};
use quicksilver::geom::{Vector};
// use std::{
//    collections::{VecDeque},
//};

pub struct World {
   m_phys: Phys,
   m_player: u64,
   // m_tick_count: u64,
   // m_enemies: VecDeque<u64>,
   // m_enemy_spawn_tick: u64,
}

impl World {
   pub fn new() -> Self {
      
      let mut phys = Phys::new().with_gravity(Vector::new(0.0, 16.0));

      let player = phys
         .create_body()
         .set_pos(Vector::new(400.0, 516.0))
         .set_vel(Vector::new(0.0, 0.0))
         .set_radius(64.0)
         .set_mass(1.0)
         .set_as_player()
         .id;

         Self {
            m_phys: phys, 
            m_player: player,
            // m_tick_count: 0,
            // m_enemies: VecDeque::new(),
            // m_enemy_spawn_tick: 80
         }
   }

   pub fn maintain(&mut self) {

      //let mut player = self.m_phys.get_body_mut(self.m_player).unwrap();

      //If left
      //player.apply_impulse(Vector::new(-1.0, 0.0));

      //If right
      //player.apply_impulse(Vector::new(1.0, 0.0));

      //if a/bongo drum
         //gen bullet

   }

   // pub fn tick(&mut self, dt: f32) {
   //    //Create enemies on timer based system
   // }

   pub fn get_player_position(&self) -> Vector {
      self.m_phys.get_body(self.m_player).unwrap().pos
   }
}