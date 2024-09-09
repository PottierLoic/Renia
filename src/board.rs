use crate::*;

pub struct Board {
    pub tiles: Vec<f32>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: vec![1.0; constants::BOARD_SIZE * constants::BOARD_SIZE],
        }
    }
}
