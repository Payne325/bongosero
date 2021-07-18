use quicksilver as qs;
use quicksilver::geom::Vector;
pub struct UserCommand {
   pub m_move_dir: Vector,
   pub m_fire_bullet: bool,
   pub m_pos: Vector,
}

impl UserCommand {
   #[allow(dead_code)]
   pub fn new_velocity_based(dir: Vector, fire: bool) -> UserCommand {
      UserCommand {
         m_move_dir: dir,
         m_fire_bullet: fire,
         m_pos: Vector::ZERO 
      }
   }

   #[allow(dead_code)]
   pub fn new_positional_based(dir: Vector, pos: Vector, fire: bool) -> UserCommand {
      UserCommand {
         m_move_dir: dir,
         m_fire_bullet: fire,
         m_pos: pos,
      }
   }
}

pub trait InputDevice {
   //Todo, remove qs input from args. Only there to read keyboard
   fn poll(&mut self, input: &qs::Input) -> UserCommand; 
   fn debug_print(&self);
}