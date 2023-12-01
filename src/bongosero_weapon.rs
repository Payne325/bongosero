use crate::input_device;
use gc_adapter::GcAdapter;
use quicksilver as qs;
use quicksilver::geom::Vector;

pub struct BongoseroWeapon {
    m_firing: bool,
    m_adapter: GcAdapter<gc_adapter::LibUsbAdapter<gc_adapter::rusb::GlobalContext>>,
}

impl BongoseroWeapon {
    pub fn new() -> Self {
        // get adapter from global context
        let adapter_wrapped = GcAdapter::from_usb();

        match adapter_wrapped {
            Some(_) => {}
            None => println!("Gamecube controller adapter not detected!"),
        }

        let adapter = adapter_wrapped.unwrap();

        BongoseroWeapon {
            m_firing: false,
            m_adapter: adapter,
        }
    }
}

impl input_device::InputDevice for BongoseroWeapon {
    fn poll(&mut self, _input: &qs::Input) -> input_device::UserCommand {
        // Would ensure controller values are absolutely correct but slows down execution significantly.
        // Doesn't appear to be necessary.
        //self.m_adapter.refresh_inputs();

        let controllers = self.m_adapter.read_controllers();

        let mut fire_weapon_down = false;
        let controller = &controllers[0];

        // Find first controller and read its button status
        if controller.connected() {
            let buttons = &controller.buttons;
            // Each GC bongo drum has two buttons. All four buttons correspond to one of abxy.
            fire_weapon_down = buttons.a() || buttons.b() || buttons.x() || buttons.y();
        }

        let shoot = |firing: bool, fire_weapon_down: bool| -> bool {
            if !firing && fire_weapon_down {
                return true;
            }
            false
        }(self.m_firing, fire_weapon_down);

        self.m_firing = fire_weapon_down;

        input_device::UserCommand::new_positional_based(Vector::ZERO, Vector::ZERO, shoot)
    }

    fn debug_print(&self) {
        if self.m_firing {
            println!("Bongo Button Pressed Down!");
        }
    }
}
