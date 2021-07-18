extern crate f_trak;
use quicksilver as qs;
use quicksilver::geom::Vector;
use crate::input_device;
use std::thread;
use std::sync::mpsc;

type Boundingbox = ((i32, i32), (i32, i32));

// Todo: these properties can live somewhere in a singleton/global space.
// Maybe they can be stored in a settings file? JSON?
const MOVE_DEAD_ZONE: f32 = 3.0;
const SCREEN : (i32, i32) = (800, 600); //Game screen coord space
const F_TRAK_MAX_BNDS : (i32, i32) = (600, 420); //f-trak coord space -> Todo. Create some sort of calibration routine to get this

pub struct BongoseroMovement {
   m_terminated : bool,
   m_bbox_receiver : mpsc::Receiver<Boundingbox>,
   m_terminate_receiver : mpsc::Receiver<bool>,
   m_prev_position : f32
}

impl BongoseroMovement {
   pub fn new() -> Self {

      //Todo: relative path -> JSON File?
      let protopath = "D:/Portfolio/f-trak/f-trak-test/static/deploy.prototxt.txt".to_string();
      let modelpath = "D:/Portfolio/f-trak/f-trak-test/static/model.caffemodel".to_string();
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

      BongoseroMovement {
         m_terminated: false, 
         m_bbox_receiver: bbox_receiver,
         m_terminate_receiver: terminate_receiver,
         m_prev_position : -1.0,
      }
   }

   fn f_trak_to_screen_coords(&self, position: f32) -> f32 {

      let edge_of_screen_offset = 20; //Handle literal edge case
      let min = 0.0 + edge_of_screen_offset as f32; 
      let max = (F_TRAK_MAX_BNDS.0 - edge_of_screen_offset) as f32;
      let norm_pos = (position - min) / (max - min);

      //multiply to move to new coord system
      norm_pos * SCREEN.0 as f32
   }

   fn get_current_position(&self) -> f32 {
      
      let mut new_pos = -1.0;
      let current_bbox = self.m_bbox_receiver.try_recv();

      match current_bbox {
         Ok(t) => {
            new_pos = self.f_trak_to_screen_coords((t.1.0 as f32 + t.0.0 as f32) / 2.0);
         },
         Err(_) => { /*println!("ERROR: {}", e);*/ },
      }

      new_pos
   }
}

// Polls f-trak for player face location and converts to game coords
impl input_device::InputDevice for BongoseroMovement {
   fn poll(&mut self, _input: &qs::Input) -> input_device::UserCommand {    
      //Get face location from f-trak
      let new_pos = self.get_current_position();

      //if this is the first recorded face position, 
      //or if the movement exceeds deadzone, them record the new position
      if self.m_prev_position == -1.0 {
         self.m_prev_position = new_pos;
      }

      if new_pos != -1.0 &&
         (new_pos - self.m_prev_position).abs() > MOVE_DEAD_ZONE { 
         self.m_prev_position = new_pos;      
      }      

      //Handle possible f-trak termination
      let cancel = self.m_terminate_receiver.try_recv();

      match cancel {
         Ok(terminate_flag) => if terminate_flag { self.m_terminated = true; },
         Err(_) => { /*println!("ERROR: {}", e);*/ },
      }

      input_device::UserCommand::new_experiment(
         Vector::ZERO, 
         Vector::new(self.m_prev_position as f32, 516.0), 
         false)
   }

   fn debug_print(&self) {
      //Todo: Further f-trak related debug information
      if self.m_terminated {
         println!("f-trak thread was terminated!")
      }
      else {
         println!("f-trak found face at {:?}", self.m_prev_position);
      }
   }
}
