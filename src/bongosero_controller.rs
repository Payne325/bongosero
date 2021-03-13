extern crate f_trak;
use quicksilver as qs;
use quicksilver::geom::Vector;
use crate::controller;
use std::thread;
use std::sync::mpsc;

type Boundingbox = ((i32, i32), (i32, i32));

// Todo: these properties can live somewhere in a singleton/global space.
// Maybe they can be stored in a settings file? JSON?

const MOVE_DEAD_ZONE: f32 = 3.0;

const SCREEN : (i32, i32) = (800, 600); //Game screen coord space
const F_TRAK_MAX_BNDS : (i32, i32) = (600, 420); //f-trak coord space -> Todo. Create some sort of calibration routine to get this
//const TRANS_FTRAK_TO_SCREEN_X: f32 = 1.333; //Conversation factor from f-trak to screen (x)

pub struct BongoseroController {
   m_a: bool,
   m_firing: bool,
   m_terminated : bool,
   m_bbox_receiver : mpsc::Receiver<Boundingbox>,
   m_terminate_receiver : mpsc::Receiver<bool>,
   m_prev_position : f32,
   m_velocity_history : Vec<f32>,
   m_speed: f32,
   m_begun: bool
}

fn f_trak_to_screen_coords(position: f32) -> f32 {
   
   let edge_of_screen_offset = 20; //Handle literal edge case
   let min = 0.0 + edge_of_screen_offset as f32; 
   let max = (F_TRAK_MAX_BNDS.0 - edge_of_screen_offset) as f32;
   let norm_pos = (position - min) / (max - min);

   //multiply to move to new coord system
   norm_pos * SCREEN.0 as f32
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
      let initial_position : f32;

      loop {
         let val = bbox_receiver.try_recv();

         match val {
            Ok(t) => { 
               initial_position = f_trak_to_screen_coords((t.1.0 as f32 + t.0.0 as f32) / 2.0);
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
         m_velocity_history : Vec::with_capacity(8),
         m_speed: 200.0,
         m_begun: false
      }
   }
   
   fn poll(&mut self, input: &qs::Input) -> controller::UserCommand {
           
      if !self.m_begun {
         //Used to place player at correct position for their given face location
         self.m_begun = true;
         return controller::UserCommand::new_experiment(
            Vector::new(0.0, 0.0), 
            Vector::new(self.m_prev_position as f32, 516.0), 
            false)
      }

      self.m_a = input.key_down(qs::input::Key::Space);
      
      //Get the move_dir from f-trak
      let mut move_dir = Vector::new(0.0, 0.0);
      let current_bbox = self.m_bbox_receiver.try_recv();

      match current_bbox {
         Ok(t) => {
            let new_pos = f_trak_to_screen_coords((t.1.0 as f32 + t.0.0 as f32) / 2.0);
            let move_vec = new_pos - self.m_prev_position;

            if self.m_velocity_history.len() == 8 {
               self.m_velocity_history.pop();
            }

            self.m_velocity_history.push(move_vec);

            let mut total_vel_history = 0.0;

            for i in 0..self.m_velocity_history.len() {
               total_vel_history += self.m_velocity_history[i];
            }

            total_vel_history = total_vel_history/self.m_velocity_history.len() as f32;

            if total_vel_history.abs() > MOVE_DEAD_ZONE { 

               self.m_velocity_history.clear();
               move_dir = Vector::new(total_vel_history as f32, 0.0);

               move_dir *= self.m_speed;

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

      controller::UserCommand::new_experiment(
         move_dir, 
         Vector::new(self.m_prev_position as f32, 516.0), 
         false)
   }

   fn debug_print(&self) {
      if self.m_a {
         println!("Space Button Pressed Down!");
      }

      //Todo: Further f-trak related debug information
      if self.m_terminated {
         println!("f-trak thread was terminated!")
      }
      else {
         println!("f-trak found face at {:?}", self.m_prev_position);
      }
   }
}
