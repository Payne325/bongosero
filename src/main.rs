mod controller;

fn main() {
    
    let mut bongo = controller::Controller::new();
    
    loop {
        bongo.print();
    }
}
