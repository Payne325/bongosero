use crate::phys::{Phys};
use crate::controller::UserCommand;
use quicksilver::geom::{Vector};

pub struct World {
   m_phys: Phys,
   m_player: u64,
   // m_enemies: VecDeque<u64>,
   // m_enemy_spawn_tick: u64,
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
            // m_enemies: VecDeque::new(),
            // m_enemy_spawn_tick: 80
         }
   }

   pub fn maintain(&mut self, command: UserCommand) {
      {
         let player_speed = 400.0;
         let mut player = self.m_phys.get_body_mut(self.m_player).unwrap();
         player.set_vel(command.m_move_dir * player_speed);
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

   // pub fn tick(&mut self, dt: f32) {
   //    //Create enemies on timer based system
   // }

   pub fn phys(&self) -> &Phys {
      &self.m_phys
   }
   
   pub fn get_player_position(&self) -> Vector {
      self.m_phys.get_body(self.m_player).unwrap().pos
   }
}