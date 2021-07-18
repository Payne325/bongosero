use quicksilver as qs;
use quicksilver::geom::Vector;
pub struct UserCommand {
   pub m_move_dir: Vector,
   pub m_fire_bullet: bool,
   pub m_pos_EXPERIMENT: Vector,
}

impl UserCommand {
   pub fn new(dir: Vector, fire: bool) -> UserCommand {

      UserCommand {
         m_move_dir: dir,
         m_fire_bullet: fire,
         m_pos_EXPERIMENT: Vector::ZERO 
      }
   }

   pub fn new_experiment(dir: Vector, pos: Vector, fire: bool) -> UserCommand {

      UserCommand {
         m_move_dir: dir,
         m_fire_bullet: fire,
         m_pos_EXPERIMENT: pos,
      }
   }
}

pub trait InputDevice {
   //Todo, remove qs input from args. The controller should be able to 
   //query the keyboard without it.
   // /fn new() -> Self;
   fn poll(&mut self, input: &qs::Input) -> UserCommand; 
   fn debug_print(&self);
}