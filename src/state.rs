//use crate::controller;
use crate::debug_controller;
use crate::world;
use crate::phys::{Body};
use quicksilver as qs;
use quicksilver::{geom::Vector, Input};
use std::collections::VecDeque;

pub struct State {
   //m_bongo : controller::Controller,
   m_controller: debug_controller::DebugController,
   m_world: world::World,
}

impl State {
   pub fn new() -> qs::Result<Self> {
      //let bongo = controller::Controller::new();
      let controller = debug_controller::DebugController::new();
      let world = world::World::new();

      Ok(Self {
         //m_bongo : bongo,
         m_controller: controller,
         m_world: world
      })
   }

   pub fn update(&mut self, input: &Input) {
      self.m_controller.poll(input);
      self.m_controller.print();

      let usr_in = world::UserInput::new(self.m_controller.left(), self.m_controller.right(), self.m_controller.shoot());
      self.m_world.maintain(usr_in);
      //self.m_bongo.poll();
      //self.m_bongo.print();
   }

   pub fn position_data(&self) -> Vector {
      //Expand this to return struct of player, bullet and enemy data
      self.m_world.get_player_position()
   }

   pub fn bullets(&self) -> VecDeque<Vector> {
      let bodies = self.m_world.phys().bodies();

      let mut positions: VecDeque<Vector> = VecDeque::new();

      for body in bodies {
         if !body.is_player{
            positions.push_back(body.pos);
         }
      }

      positions
   }
}