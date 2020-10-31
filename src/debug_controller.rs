use quicksilver as qs;
use quicksilver::geom::Vector;

pub struct UserCommand {
   pub m_move_dir: Vector,
   pub m_fire_bullet: bool,
}

impl UserCommand {
   pub fn new(dir: Vector, fire: bool) -> UserCommand {

      UserCommand {
         m_move_dir: dir,
         m_fire_bullet: fire,
      }
   }
}

pub struct DebugController {
   m_a: bool,
   m_b: bool,
   m_x: bool,
   m_y: bool,
   m_left: bool,
   m_right: bool,
   m_firing: bool
}

// Controller used for debuging the game without the bongo controller, keyboard
impl DebugController {
   pub fn new() -> Self {
      Self {
         m_a: false,
         m_b: false,
         m_x: false,
         m_y: false,
         m_left: false,
         m_right: false,
         m_firing: false
      }
   }

   pub fn poll(&mut self, input: &qs::Input) -> UserCommand {
           
      self.m_a = input.key_down(qs::input::Key::Space);
      self.m_b = input.key_down(qs::input::Key::X);
      self.m_x = input.key_down(qs::input::Key::C);
      self.m_y = input.key_down(qs::input::Key::V);
      self.m_left = input.key_down(qs::input::Key::Left);
      self.m_right = input.key_down(qs::input::Key::Right);

      let mut move_dir = Vector::new(0.0, 0.0);

      if self.m_left && !self.m_right {
         move_dir += Vector::new(-1.0, 0.0);
      }
      else if !self.m_left && self.m_right {
         move_dir += Vector::new(1.0, 0.0);
      }

      let shoot = |firing: bool, a_down: bool| -> bool {
         if !firing && a_down {
            return true
         }        
         false
      }(self.m_firing, self.m_a);

      self.m_firing = self.m_a;

      UserCommand::new(move_dir, shoot)
   }

   pub fn print(&self) {
      if self.m_a {
         println!("A Button Pressed");
      }

      if self.m_b {
         println!("B Button Pressed");
      }

      if self.m_x {
         println!("X Button Pressed");
      }

      if self.m_y {
         println!("Y Button Pressed");
      }

      if self.m_left {
         println!("Left Dpad Button Pressed!");
      }

      if self.m_right {
         println!("Right Dpad Button Pressed");
      }
   }
}
