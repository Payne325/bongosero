use crate::input_device::InputDevice;

#[cfg(feature = "keyboard")]
use crate::keyboard_weapon::KeyboardWeapon;
#[cfg(feature = "keyboard")]
use crate::keyboard_movement::KeyboardMovement;

#[cfg(not(feature = "keyboard"))]
use crate::bongosero_movement::BongoseroMovement;
#[cfg(not(feature = "keyboard"))]
use crate::bongosero_weapon::BongoseroWeapon;

use crate::world;
use quicksilver as qs;
use quicksilver::{
   geom::{Rectangle, Vector}, 
   graphics::{Color, Image},
   Input, 
   Graphics
};
use std::collections::VecDeque;

pub struct Game {
   m_weapon_device: Box<dyn InputDevice>,
   m_move_device: Box<dyn InputDevice>,
   m_world: world::World,
   m_background: Image,
   m_player_sprite: Image,
   m_bullet_sprite: Image,
   m_start_msg: Image,
   m_background_region: Rectangle,
   m_game_has_begun : bool
}

impl Game {
   #[cfg(feature = "keyboard")]
   fn construct_weapon_device() -> Box<dyn InputDevice> {    
      Box::new(KeyboardWeapon::new())
   }
   
   #[cfg(feature = "keyboard")]
   fn construct_move_device() -> Box<dyn InputDevice> {   
      Box::new(KeyboardMovement::new())
   }

   #[cfg(not(feature = "keyboard"))]
   fn construct_weapon_device() -> Box<dyn InputDevice> {    
      Box::new(BongoseroWeapon::new())
   }

   #[cfg(not(feature = "keyboard"))]
   fn construct_move_device() -> Box<dyn InputDevice> {   
      Box::new(BongoseroMovement::new())
   }

   pub fn new(background: Image, player: Image, bullet: Image, start_msg: Image) -> qs::Result<Self> {
      let weapon_device = Game::construct_weapon_device();
      let move_device = Game::construct_move_device();
      let world = world::World::new();

      let background_region = Rectangle::new(Vector::new(0.0, 0.0), background.size());

      Ok(Game {
         m_weapon_device: weapon_device,
         m_move_device: move_device,
         m_world: world,
         m_background: background,
         m_player_sprite: player,
         m_bullet_sprite: bullet,
         m_start_msg: start_msg,
         m_background_region: background_region,
         m_game_has_begun: false
      })
   }

   pub fn update(&mut self, input: &Input) {
      let mut user_commands = self.m_move_device.poll(input);
      user_commands.m_fire_bullet = self.m_weapon_device.poll(input).m_fire_bullet;

      if !self.m_game_has_begun {
         if user_commands.m_fire_bullet {
            self.m_game_has_begun = true;
         }
         
         return;
      }

      if cfg!(feature = "debug") {
         self.m_move_device.debug_print();
         self.m_weapon_device.debug_print();
      }

      self.m_world.maintain(user_commands);
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

      if !self.m_game_has_begun {
         let region = Rectangle::new(Vector::new(254.0, 243.0), self.m_start_msg.size());
         gfx.draw_image(&self.m_start_msg, region);
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