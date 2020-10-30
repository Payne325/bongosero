use quicksilver as qs;

pub struct DebugController {
   m_a: bool,
   m_b: bool,
   m_x: bool,
   m_y: bool,
   m_left: bool,
   m_right: bool
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
         m_right: false
      }
   }

   pub fn poll(&mut self, input: &qs::Input) {
      self.m_a = input.key_down(qs::input::Key::Space);
      self.m_b = input.key_down(qs::input::Key::X);
      self.m_x = input.key_down(qs::input::Key::C);
      self.m_y = input.key_down(qs::input::Key::V);
      self.m_left = input.key_down(qs::input::Key::Left);
      self.m_right = input.key_down(qs::input::Key::Right);
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
