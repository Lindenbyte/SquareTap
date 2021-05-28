use std::{fs, path::Path};

use macroquad::prelude::*;

use serde_derive::Deserialize;
use toml;


#[derive(Deserialize)]
struct GameOptions {
    pub tile_filled: Vec<u8>,
    pub tile_empty: Vec<u8>,
    pub tile_border: Vec<u8>,
    pub scale: f64,
}

pub struct GameSettings {
    pub filled_color: Color,
    pub empty_color: Color,
    pub border_color: Color,
    pub scale: f32,
}

const TILE_FILLED_COLOR: Color = color_u8!(45, 55, 65, 255);
const TILE_EMPTY_COLOR: Color = color_u8!(40, 130, 115, 255);
const TILE_BORDER_COLOR: Color = color_u8!(255, 255, 255, 255);

const DEFAULT_GAME_OPTIONS: GameSettings = GameSettings{
    filled_color: TILE_FILLED_COLOR,
    empty_color: TILE_EMPTY_COLOR,
    border_color: TILE_BORDER_COLOR,
    scale: 1.0
};


pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }
    
    pub fn load_options() -> GameSettings {
        if Path::new("gamesave.bin").exists() {
            let options_str = fs::read_to_string("settings.toml").unwrap();
            let game_options: GameOptions = toml::from_str(&*options_str).unwrap();

            return GameSettings {
                filled_color: color_u8!(
                    game_options.tile_filled[0],
                    game_options.tile_filled[1],
                    game_options.tile_filled[2],
                    game_options.tile_filled[3]
                ),
                empty_color: color_u8!(
                    game_options.tile_empty[0],
                    game_options.tile_empty[1],
                    game_options.tile_empty[2],
                    game_options.tile_empty[3]
                ),
                border_color: color_u8!(
                    game_options.tile_border[0],
                    game_options.tile_border[1],
                    game_options.tile_border[2],
                    game_options.tile_border[3]
                ),
                scale: game_options.scale as f32
            };
        } else {
            return DEFAULT_GAME_OPTIONS;
        }
    }
    
    pub fn update(&mut self) {}

    pub fn render(&mut self, font: Font) {
        let title = "Settings";
        let title_dimensions = measure_text(title, Some(font), 78, 1.0);
        draw_text_ex(
            title,
            screen_width() / 2.0 - title_dimensions.width / 2.0,
            screen_height() / 2.0 - 250.0,
            TextParams {
                font,
                font_size: 78,
                color: WHITE,
                ..Default::default()
            },
        );

        // Key bindings
        //  Menu
        draw_text_ex(
            "Key bindnings:",
            250.0,
            screen_height() / 2.0 - 175.0,
            TextParams {
                font,
                font_size: 32,
                color: WHITE,
                ..Default::default()
            },
        );

        draw_text_ex(
            "Select: Enter",
            250.0,
            screen_height() / 2.0 - 125.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Back: Escape",
            250.0,
            screen_height() / 2.0 - 100.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Move up: ArrowUp",
            250.0,
            screen_height() / 2.0 - 75.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Move down: ArrowDown",
            250.0,
            screen_height() / 2.0 - 50.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );

        draw_text_ex(
            "Hide info: Tab",
            250.0,
            screen_height() / 2.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            "Hide grid: G",
            250.0,
            screen_height() / 2.0 + 25.0,
            TextParams {
                font,
                font_size: 18,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}

impl Default for Settings {
    fn default() -> Settings {
        return Settings {};
    }
}
