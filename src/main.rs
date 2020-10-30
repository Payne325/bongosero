mod state;
mod debug_controller;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
};

fn main() {
    run::<state::State>("Bongosero", Vector::new(800, 600), Settings::default());
}
