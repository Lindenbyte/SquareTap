extern crate savefile;
use savefile::prelude::*;

#[derive(Savefile)]
pub struct GameSave {
    pub highscores: Vec<u32>,
}

impl GameSave {
    pub fn new() -> Self {
        return Self {
            highscores: vec![0; 9],
        };
    }

    pub fn new_from_data(highscores: Vec<u32>) -> Self {
        return Self {
            highscores: highscores,
        };
    }

    pub fn save_to_file(&mut self) {
        save_file("gamesave.bin", 0, self).unwrap();
    }

}

pub fn load_from_file() -> GameSave {
    return load_file("gamesave.bin", 0).unwrap();
}