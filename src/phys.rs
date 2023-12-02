use quicksilver::geom::Vector;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

pub struct Body {
    pub id: u64,
    pub pos: Vector,
    pub vel: Vector,
    pub force_record: Vector,
    pub radius: f32,
    pub mass: f32,
    pub is_player: bool,
}

impl Body {
    pub fn set_as_player(&mut self) -> &mut Self {
        self.is_player = true;
        self
    }

    pub fn set_pos(&mut self, pos: Vector) -> &mut Self {
        self.pos = pos;
        self
    }

    pub fn set_vel(&mut self, vel: Vector) -> &mut Self {
        self.vel = vel;
        self
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    fn step(&mut self, dt: f32) {
        self.vel += self.force_record / self.mass;
        self.force_record = Vector::ZERO;

        self.pos += self.vel * dt;
    }
}

pub struct Phys {
    id_counter: u64,
    bodies: HashMap<u64, RefCell<Body>>,
    gravity: Vector,
}

impl Phys {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            bodies: HashMap::new(),
            gravity: Vector::ZERO,
        }
    }

    pub fn with_gravity(mut self, gravity: Vector) -> Self {
        self.gravity = gravity;
        self
    }
    pub fn create_body(&mut self) -> RefMut<Body> {
        let id = self.new_id();

        self.bodies
            .entry(id)
            .or_insert(RefCell::new(Body {
                id,
                pos: Vector::ZERO,
                vel: Vector::ZERO,
                force_record: Vector::ZERO,
                radius: 16.0,
                mass: 1.0,
                is_player: false,
            }))
            .borrow_mut()
    }

    pub fn get_body(&self, id: u64) -> Option<Ref<Body>> {
        self.bodies.get(&id).map(|b| b.borrow())
    }

    pub fn get_body_mut(&mut self, id: u64) -> Option<RefMut<Body>> {
        self.bodies.get(&id).map(|b| b.borrow_mut())
    }

    pub fn bodies_mut(&self) -> impl Iterator<Item = RefMut<Body>> {
        self.bodies.values().map(|b| b.borrow_mut())
    }

    fn new_id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn tick(&mut self, dt: f32) {
        // Propagate positions
        for mut body in self.bodies_mut() {
            body.step(dt);
        }
    }

    pub fn delete_body(&mut self, id: u64) -> bool {
        self.bodies.remove(&id).is_some()
    }
}
