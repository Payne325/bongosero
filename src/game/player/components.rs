use std::{path::Path, sync::mpsc, thread, time::Duration};

use bevy::prelude::*;
use bevy_aseprite::aseprite;

aseprite!(pub PlayerAseprite, "sprites/santa_crack.aseprite");

const SHOOT_ANIM_TIME_SECS: f32 = 1.0/(12.0/7.0);

#[derive(Component)]
pub struct Player {
    pub animation_timer: Timer,
}

impl Default for Player{
    fn default() -> Self {
        let animation_timer = Timer::new(Duration::from_secs_f32(SHOOT_ANIM_TIME_SECS), TimerMode::Once);
        Self { animation_timer }
    }
}

type Boundingbox = ((i32, i32), (i32, i32));

// Not deriving component as I want to access this as NonSendMut
pub struct FaceTracker {
    pub bbox_receiver: mpsc::Receiver<Boundingbox>,
    pub terminate_receiver: mpsc::Receiver<bool>,
}

impl Default for FaceTracker {
    fn default() -> FaceTracker {
        let protopath = Path::new("./assets/neural_nets/face_detection/deploy.prototxt.txt")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let modelpath = Path::new("./assets/neural_nets/face_detection/model.caffemodel")
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .to_string();

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
            terminate_receiver,
        }
    }
}
