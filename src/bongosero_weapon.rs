use crate::input_device;
use quicksilver as qs;
use quicksilver::geom::Vector;
use gc_adapter::GcAdapter;

pub struct BongoseroWeapon {
   m_firing : bool,
   m_adapter : GcAdapter<gc_adapter::LibUsbAdapter<gc_adapter::rusb::GlobalContext>>
}

impl input_device::InputDevice for BongoseroWeapon {

   fn new() -> Self {

      // get adapter from global context
      let adapter_wrapped = GcAdapter::from_usb();

      match adapter_wrapped {
         Some(_) => {}, 
         None => println!("Gamecube controller adapter not detected!"),
      }

      let adapter = adapter_wrapped.unwrap();
      
      BongoseroWeapon {
         m_firing : false,
         m_adapter : adapter,
      }
   }


   fn poll(&mut self, input: &qs::Input) -> input_device::UserCommand {

      self.m_adapter.refresh_inputs(); 
      let controllers = self.m_adapter.read_controllers();

      let mut fire_weapon_down = false;
      for controller in controllers {
         //Simplest implementation for now. 
         //Find first connected controller and read its button status
         if controller.connected() {
            let buttons = controller.buttons;

            //Each GC bongo drum has two buttons. All four of which correspond to one of abxy. 
            fire_weapon_down = buttons.a() || buttons.b() || buttons.x() || buttons.y();
            break;            
         }
      }

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
         println!("Bongo Button Pressed Down!");
      }
   }
}