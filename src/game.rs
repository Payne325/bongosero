use crate::input_device::InputDevice;
use crate::bongosero_movement;
use crate::debug_weapon;
//use crate::debug_movement;
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
   //m_bongo : controller::Controller,
   m_weapon_device: debug_weapon::DebugWeapon,
   m_move_device: bongosero_movement::BongoseroMovement,
   m_world: world::World,
   m_background: Image,
   m_player_sprite: Image,
   m_bullet_sprite: Image,
   m_background_region: Rectangle
}

impl Game {
   pub fn new(background: Image, player: Image, bullet: Image) -> qs::Result<Self> {
      //let bongo = controller::Controller::new();
      let weapon_device = debug_weapon::DebugWeapon::new();
      let move_device = bongosero_movement::BongoseroMovement::new();
      let world = world::World::new();

      let background_region = Rectangle::new(Vector::new(0.0, 0.0), background.size());

      Ok(Game {
         //m_bongo : bongo,
         m_weapon_device: weapon_device,
         m_move_device: move_device,
         m_world: world,
         m_background: background,
         m_player_sprite: player,
         m_bullet_sprite: bullet,
         m_background_region: background_region
      })
   }

   pub fn update(&mut self, input: &Input) {
      let mut user_commands = self.m_move_device.poll(input);
      user_commands.m_fire_bullet = self.m_weapon_device.poll(input).m_fire_bullet;

      //Todo: Add debug flag to control printing controller poll results
      self.m_move_device.debug_print();
      self.m_weapon_device.debug_print();
      
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