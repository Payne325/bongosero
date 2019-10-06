extern crate gcnctrlusb;

fn main() {
    // Panics if `libusb` is not found or otherwise fails.
    let mut scanner = gcnctrlusb::Scanner::new().unwrap();

    // Panics if a valid device was not found.
    let mut adapter = scanner.find_adapter().unwrap().unwrap();

    // Panics if the USB driver fails to open a connection to the device.
    let mut listener = adapter.listen().unwrap();

    while let Ok(controllers) = listener.read() {

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
