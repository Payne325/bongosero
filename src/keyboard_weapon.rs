use crate::input_device;
use quicksilver as qs;
use quicksilver::geom::Vector;

pub struct KeyboardWeapon {
   m_firing: bool
}

impl KeyboardWeapon {
   pub fn new() -> Self { 
      KeyboardWeapon {          
         m_firing: false,
      }
   }
}

//Input device to handle player weapons fire, debug device is keyboard
impl input_device::InputDevice for KeyboardWeapon {
   fn poll(&mut self, input: &qs::Input) -> input_device::UserCommand {
      let fire_weapon_down = input.key_down(qs::input::Key::Space);
   
      let shoot = |firing: bool, fire_weapon_down: bool| -> bool {
         if !firing && fire_weapon_down {
            return true
         }        
         false
      }(self.m_firing, fire_weapon_down);
   
      self.m_firing = fire_weapon_down;

      input_device::UserCommand::new_velocity_based(Vector::ZERO, shoot)
   }

   fn debug_print(&self) {
      if self.m_firing {
         println!("Space Button Pressed Down!");
      }
   }
}