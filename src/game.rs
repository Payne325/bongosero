//use crate::controller;
use crate::debug_controller;
use crate::world;
use crate::phys::{Body};
use quicksilver as qs;
use quicksilver::{
   geom::{Rectangle, Vector}, 
   graphics::{Color, Image},
   Input, 
   Graphics
};
use std::collections::VecDeque;

pub struct Game {
   //m_bongo : controller::Controller,
   m_controller: debug_controller::DebugController,
   m_world: world::World,
   m_background: Image,
   m_player_sprite: Image,
   m_bullet_sprite: Image,
   m_background_region: Rectangle
}

impl Game {
   pub fn new(background: Image, player: Image, bullet: Image) -> qs::Result<Self> {
      //let bongo = controller::Controller::new();
      let controller = debug_controller::DebugController::new();
      let world = world::World::new();

      let background_region = Rectangle::new(Vector::new(0.0, 0.0), background.size());

      Ok(Game {
         //m_bongo : bongo,
         m_controller: controller,
         m_world: world,
         m_background: background,
         m_player_sprite: player,
         m_bullet_sprite: bullet,
         m_background_region: background_region
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

   pub fn draw(&mut self, mut gfx: Graphics) -> Graphics { 
      gfx.clear(Color::BLACK);

      gfx.draw_image(&self.m_background, self.m_background_region);

      let player_region = Rectangle::new(self.m_world.get_player_position(), self.m_player_sprite.size());
      gfx.draw_image(&self.m_player_sprite, player_region);

      let bullets = self.bullets();

      for b in bullets {
         let region = Rectangle::new(b, self.m_bullet_sprite.size());
         gfx.draw_image(&self.m_bullet_sprite, region);
      }

      gfx
   }

   fn bullets(&self) -> VecDeque<Vector> {
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