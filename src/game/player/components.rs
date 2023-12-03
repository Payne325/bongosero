use std::{sync::mpsc, thread};

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

type Boundingbox = ((i32, i32), (i32, i32));

// Not deriving component as I want to access this as NonSendMut
pub struct FaceTracker {
    pub bbox_receiver: mpsc::Receiver<Boundingbox>,
    pub terminate_receiver: mpsc::Receiver<bool>,
}

impl Default for FaceTracker {
    fn default() -> FaceTracker {
        let protopath = "D:/Portfolio/bongosero/assets/neural_nets/face_detection/deploy.prototxt.txt".to_string();
        let modelpath = "D:/Portfolio/bongosero/assets/neural_nets/face_detection/model.caffemodel".to_string();
        let min_confidence = 0.9;
    
        let (bbox_transmitter, bbox_receiver) = mpsc::channel::<Boundingbox>();
        let (terminate_transmitter, terminate_receiver) = mpsc::channel::<bool>();

        thread::spawn(move || {
            let mut face_cap = f_trak::FaceCapture::new(
                bbox_transmitter,
                terminate_transmitter,
                protopath,
                modelpath,
                min_confidence,
            );

            face_cap.begin_capture();
        });

        FaceTracker {
            bbox_receiver,
            terminate_receiver
        }
    }
}