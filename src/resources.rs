use bevy::prelude::*;
use gc_adapter::{Buttons, GcAdapter};

#[derive(Resource)]
pub struct Bongo {
    just_fired: bool,
    adapter: GcAdapter<gc_adapter::LibUsbAdapter<gc_adapter::rusb::GlobalContext>>,
}

impl Default for Bongo {
    fn default() -> Self {
        let adapter = GcAdapter::from_usb().unwrap();
        Self {
            just_fired: false,
            adapter,
        }
    }
}

impl Bongo {
    pub fn check_gun_fired(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.a() || buttons.b() || buttons.x() || buttons.y();

        self.check_for_condition(closure)
    }

    pub fn check_select_pressed(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.a();

        self.check_for_condition(closure)
    }

    pub fn check_back_pressed(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.b();

        self.check_for_condition(closure)
    }

    fn check_for_condition(&mut self, closure: impl Fn(&Buttons) -> bool) -> bool {
        let controllers = self.adapter.read_controllers();
        let controller = &controllers[0];

        let fire_pressed = if controller.connected() {
            let buttons = &controller.buttons;
            closure(buttons)
        } else {
            false
        };

        if self.just_fired {
            self.just_fired = false;
        } else if fire_pressed {
            self.just_fired = true;
        }

        self.just_fired
    }
}
