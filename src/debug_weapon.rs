use crate::input_device;
use quicksilver as qs;
use quicksilver::geom::Vector;

pub struct DebugWeapon {
   m_firing: bool
}

//Input device to handle player weapons fire, debug device is keyboard
impl input_device::InputDevice for DebugWeapon {
   fn new() -> Self { 
      DebugWeapon {          
         m_firing: false,
      }
   }

   fn poll(&mut self, input: &qs::Input) -> input_device::UserCommand {
      let fire_weapon_down = input.key_down(qs::input::Key::Space);
   
      let shoot = |firing: bool, fire_weapon_down: bool| -> bool {
         if !firing && fire_weapon_down {
            return true
         }        
         false
      }(self.m_firing, fire_weapon_down);
   
      self.m_firing = fire_weapon_down;

      input_device::UserCommand::new_experiment(Vector::ZERO, Vector::ZERO, shoot)
   }

   fn debug_print(&self) {
      if self.m_firing {
         println!("Space Button Pressed Down!");
      }
   }
}