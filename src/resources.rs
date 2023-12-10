use std::time::Duration;

use bevy::prelude::*;
use gc_adapter::{Buttons, GcAdapter};

const BONGO_COOLDOWN: f32 = 0.25;

#[derive(Resource)]
pub struct Bongo {
    just_fired: bool,
    timer: Timer,
    adapter: GcAdapter<gc_adapter::LibUsbAdapter<gc_adapter::rusb::GlobalContext>>,
}

impl Default for Bongo {
    fn default() -> Self {
        let adapter = GcAdapter::from_usb().unwrap();
        let timer = Timer::new(Duration::from_secs_f32(BONGO_COOLDOWN), TimerMode::Once);
        Self {
            just_fired: false,
            timer,
            adapter,
        }
    }
}

impl Bongo {
    pub fn check_gun_fired(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.a() || buttons.b() || buttons.x() || buttons.y();

        if self.check_for_condition(closure) && self.timer.finished() {
            self.timer.reset();
            true
        } else {
            false
        }
    }

    pub fn check_select_pressed(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.a();

        self.check_for_condition(closure)
    }

    pub fn check_back_pressed(&mut self) -> bool {
        let closure = |buttons: &Buttons| buttons.b();

        self.check_for_condition(closure)
    }

    pub fn tick(&mut self, time: Res<Time>) {
        self.timer.tick(time.delta());
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
