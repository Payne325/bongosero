extern crate gcnctrlusb;

pub struct Controller {
    m_scanner : gcnctrlusb::Scanner,
    m_a : bool,
    m_b : bool,
    m_x : bool,
    m_y : bool,
    m_z : bool,
    m_start : bool,
    m_r : bool
}

impl Controller {
    pub fn new() -> Self {
        // Panics if `libusb` is not found or otherwise fails.
        Self {
            m_scanner : gcnctrlusb::Scanner::new().unwrap(),
            m_a : false,
            m_b : false,
            m_x : false,
            m_y : false,
            m_z : false,
            m_start : false,
            m_r : false
        }
    }

    pub fn poll(&mut self) {
        // Panics if a valid device was not found.
        let mut adapter = self.m_scanner.find_adapter().unwrap().unwrap();

        // Panics if the USB driver fails to open a connection to the device.
        let mut listener = adapter.listen().unwrap();

        if let Ok(controllers) = listener.read() {

            let controller = controllers[0].unwrap();

            self.m_a = controller.a;
            self.m_b = controller.b;
            self.m_x = controller.x;
            self.m_y = controller.y;
            self.m_z = controller.z;
            self.m_start = controller.start;
            self.m_r = controller.r;
        }
    }

    pub fn print(& self) {

        if self.m_a {
            println!("A Button Held");
        }

        if self.m_b {
            println!("B Button Held");
        }

        if self.m_x {
            println!("X Button Held");
        }

        if self.m_y {
            println!("Y Button Held");
        }

        if self.m_z {
            println!("Z Button Held!");
        }

        if self.m_r {
            println!("Clap Detected");
        }

        if self.m_start {
            println!("Start Pressed");
        }        
    }
}
