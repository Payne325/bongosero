use bevy::prelude::*;

#[derive(Resource)]
#[derive(Default)]
pub struct Score {
    pub value: u32,
}



#[derive(Resource, Debug)]
#[derive(Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}


