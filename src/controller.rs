extern crate gcnctrlusb;

pub struct Controller {
    m_scanner : gcnctrlusb::Scanner
}

impl Controller {
    pub fn new() -> Self {
        // Panics if `libusb` is not found or otherwise fails.
        Self {
            m_scanner : gcnctrlusb::Scanner::new().unwrap()
        }
    }

    pub fn print(&mut self) {

        // Panics if a valid device was not found.
        let mut adapter = self.m_scanner.find_adapter().unwrap().unwrap();

        // Panics if the USB driver fails to open a connection to the device.
        let mut listener = adapter.listen().unwrap();

        if let Ok(controllers) = listener.read() {

            let controller = controllers[0].unwrap();

            if controller.a {
                println!("A Button Held");
            }

            if controller.b {
                println!("B Button Held");
            }

            if controller.x {
                println!("X Button Held");
            }

            if controller.y {
                println!("Y Button Held");
            }

            if controller.z {
                println!("Z Button Held!");
            }

            if controller.r {
                println!("Clap Detected");
            }

            if controller.start {
                println!("Start Pressed");
            }        
        }
    }
}