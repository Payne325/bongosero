extern crate f_trak;
use quicksilver as qs;
use quicksilver::geom::Vector;
use crate::controller;
use std::thread;
use std::sync::mpsc;

type Boundingbox = ((i32, i32), (i32, i32));

const MOVE_DEAD_ZONE: i32 = 5;

pub struct BongoseroController {
   m_a: bool,
   m_firing: bool,
   m_terminated : bool,
   m_bbox_receiver : mpsc::Receiver<Boundingbox>,
   m_terminate_receiver : mpsc::Receiver<bool>,
   m_prev_position : i32,
   m_speed : f32
}

// Controller used for debuging the game without the bongo controller, keyboard
impl controller::Controller for BongoseroController {
   fn new() -> Self {

      //Todo: relative path
      let protopath = "D:/Portfolio/f-trak/f-trak/static/deploy.prototxt.txt".to_string();
      let modelpath = "D:/Portfolio/f-trak/f-trak/static/model.caffemodel".to_string();
      let min_confidence = 0.9;

      let (bbox_transmitter, bbox_receiver) = mpsc::channel::<Boundingbox>();
      let (terminate_transmitter, terminate_receiver) = mpsc::channel::<bool>();
   
      thread::spawn(move || {
         println!("DEBUG: Spawned the face capture thread!");
         let mut face_cap = f_trak::FaceCapture::new(bbox_transmitter, 
                                                     terminate_transmitter,
                                                     protopath,
                                                     modelpath,
                                                     min_confidence);
         face_cap.begin_capture();
      });

      //Get intial position from f-trak
      let initial_position : i32;

      loop {
         let val = bbox_receiver.try_recv();

         match val {
            Ok(t) => { 
               initial_position = (t.1.0 + t.0.0) / 2;
              break;
            },
            Err(_) => { /*println!("ERROR: {}", e);*/ },
         }
      }

      BongoseroController {
         m_a: false,
         m_firing: false,
         m_terminated: false, 
         m_bbox_receiver: bbox_receiver,
         m_terminate_receiver: terminate_receiver,
         m_prev_position : initial_position,
         m_speed : 5.0
      }
   }
   
   fn poll(&mut self, input: &qs::Input) -> controller::UserCommand {
           
      self.m_a = input.key_down(qs::input::Key::Space);
      
      //Get the move_dir from f-trak
      let mut move_dir = Vector::new(0.0, 0.0);
      let current_bbox = self.m_bbox_receiver.try_recv();

      match current_bbox {
         Ok(t) => {
            let new_pos = (t.1.0 + t.0.0)/2;
            let move_vec = new_pos - self.m_prev_position;

            if move_vec.abs() > MOVE_DEAD_ZONE { 
               if move_vec > 0 {
                  move_dir += Vector::new(-1.0, 0.0);
               }
               else if move_vec < 0 {
                  move_dir += Vector::new(1.0, 0.0);
               }

               self.m_prev_position = new_pos;
            }
         },
         Err(_) => { /*println!("ERROR: {}", e);*/ },
      }

      let shoot = |firing: bool, a_down: bool| -> bool {
         if !firing && a_down {
            return true
         }        
         false
      }(self.m_firing, self.m_a);

      self.m_firing = self.m_a;

      //Handle possible f-trak termination
      let cancel = self.m_terminate_receiver.try_recv();

      match cancel {
         Ok(terminate_flag) => if terminate_flag { self.m_terminated = true; },
         Err(_) => { /*println!("ERROR: {}", e);*/ },
      }

      controller::UserCommand::new(move_dir, shoot)
   }

   fn debug_print(&self) {
      if self.m_a {
         println!("Space Button Pressed Down!");
      }

      //Todo: Further f-trak relate debug information
      if self.m_terminated {
         println!("f-trak thread was terminated!")
      }
      else {
         println!("f-trak found face at {:?}", self.m_prev_position);
      }
   }
}
