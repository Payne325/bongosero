use crate::phys::Phys;
use crate::input_device::UserCommand;
use quicksilver::geom::Vector;

pub struct World {
   m_phys: Phys,
   m_player: u64,
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

      self.m_phys.tick(1.0/60.0);

      let bullet_speed = 1000.0;

      if command.m_fire_bullet {
         let player_pos = self.get_player_position();

         self.m_phys
            .create_body()
            .set_pos(player_pos + Vector::new(16.0, -16.0)) //x coord = player.x + bullet height
            .set_vel(Vector::new(0.0, -bullet_speed))
            .set_radius(16.0)
            .set_mass(1.0);
      }
   }

   pub fn phys(&self) -> &Phys {
      &self.m_phys
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
}