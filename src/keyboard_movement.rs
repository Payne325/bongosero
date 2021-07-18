use crate::input_device;
use quicksilver as qs;
use quicksilver::geom::Vector;

const SPEED : f32 = 400.0;

pub struct KeyboardMovement {
   m_left : bool,
   m_right : bool
}

impl KeyboardMovement{
   pub fn new() -> Self { 
      KeyboardMovement {       
         m_left : false,
         m_right: false   
      }
   }
}

impl input_device::InputDevice for KeyboardMovement {
   fn poll(&mut self, input: &qs::Input) -> input_device::UserCommand {
      self.m_left = input.key_down(qs::input::Key::Left);
      self.m_right = input.key_down(qs::input::Key::Right);

      let mut move_dir = Vector::new(0.0, 0.0);

      if self.m_left && !self.m_right {
         move_dir += Vector::new(-1.0, 0.0);
      }
      else if !self.m_left && self.m_right {
         move_dir += Vector::new(1.0, 0.0);
      }

      input_device::UserCommand::new(move_dir*SPEED, false)
   }

   fn debug_print(&self) {

      if self.m_left {
         println!("Left Dpad Button Pressed!");
      }

      if self.m_right {
         println!("Right Dpad Button Pressed");
      }
   }
}