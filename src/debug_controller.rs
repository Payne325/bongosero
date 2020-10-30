use quicksilver as qs;
use quicksilver::input::{ButtonState, Gamepad, GamepadButton};

pub struct Debug_Controller {
   m_a: bool,
   m_b: bool,
   m_x: bool,
   m_y: bool,
   m_left: bool,
   m_right: bool,
   m_gamepads: Vec<Gamepad>,
   m_gamepad_found: bool
}

// Controller used for debuging the game without the bongo controller,
// Configured with xbox controller in mind.
impl Debug_Controller {
   pub fn new() -> Self {
      Self {
         m_a: false,
         m_b: false,
         m_x: false,
         m_y: false,
         m_left: false,
         m_right: false,
         m_gamepads: Vec::new(),
         m_gamepad_found: false
      }
   }

   pub fn poll(&mut self, window: &mut qs::lifecycle::Window) {
      if self.m_gamepad_found == false {
         self.m_gamepads = window.gamepads().to_vec();
         
         if self.m_gamepads.len() > 0 {
            println!("Gamepad found");
            self.m_gamepad_found = true;
         }
      }

      if self.m_gamepad_found {
         ()
      }

      self.m_a = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::FaceDown] == ButtonState::Pressed);
      self.m_b = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::FaceRight] == ButtonState::Pressed);
      self.m_x = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::FaceLeft] == ButtonState::Pressed);
      self.m_y = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::FaceUp] == ButtonState::Pressed);
      self.m_left = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::DpadLeft] == ButtonState::Pressed);
      self.m_right = self
         .m_gamepads
         .iter()
         .any(|pad| pad[qs::input::GamepadButton::DpadRight] == ButtonState::Pressed);
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
