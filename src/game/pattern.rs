use macroquad::prelude::*;

const SCALE_MIN: f32 = 0.5;
const SCALE_MAX: f32 = 3.0;
const SCALE_CHANGE: f32 = 0.25;

pub struct Pattern {
    pub score: u32,
    pub multiplier: u16,
    scale: f32,
    display_info: bool,
    tiles: [bool; 16],
}

impl Pattern {
    pub fn new() -> Self {
        // return Self { ..Default::default() };
        return Self { tiles: [false, true, false, true, true, false, true, false, false, true, false, true, true, false, true, false], ..Default::default() };
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            self.display_info = !self.display_info;
        }

        // Scale
        if is_key_pressed(KeyCode::PageUp) {
            self.scale += SCALE_CHANGE;
        } else if is_key_pressed(KeyCode::PageDown) {
            self.scale -= SCALE_CHANGE;
        }

        if self.scale > SCALE_MAX {
            self.scale = SCALE_MAX;
        } else if self.scale < SCALE_MIN {
            self.scale = SCALE_MIN;
        }
    }

    pub fn render(&mut self) {
        // display_info information
        if self.display_info {
            draw_text(&*format!("Multiplier: {}", self.multiplier), 50.0, 100.0, 16.0, WHITE);
            draw_text(&*format!("Scale: {}", self.scale), 50.0, 120.0, 16.0, WHITE);
        }
        
        // Tiles and grid
        for tile in 0..16 {
            let size: f32 = 250.0 * self.scale;
            let filled: bool = self.tiles[tile];

            let filled_color: Color = Color::from_rgba(30, 210, 255, 255);
            let empty_color: Color = Color::from_rgba(255, 190, 30, 255);
            let color: Color;

            let xy = (
                (screen_width() / 2.0) - size / 2.0, 
                (screen_height() / 2.0) - size / 2.0
            );
            let cell = size / 4.0;

            if filled {
                color = filled_color;
            } else {
                color = empty_color;
            }

            // Tiles
            match tile {
                0 => draw_rectangle(xy.0,               xy.1, cell, cell, color),
                1 => draw_rectangle(xy.0 + cell,        xy.1, cell, cell, color),
                2 => draw_rectangle(xy.0 + cell * 2.0,  xy.1, cell, cell, color),
                3 => draw_rectangle(xy.0 + cell * 3.0,  xy.1, cell, cell, color),

                4 => draw_rectangle(xy.0,               xy.1 + cell, cell, cell, color),
                5 => draw_rectangle(xy.0 + cell,        xy.1 + cell, cell, cell, color),
                6 => draw_rectangle(xy.0 + cell * 2.0,  xy.1 + cell, cell, cell, color),
                7 => draw_rectangle(xy.0 + cell * 3.0,  xy.1 + cell, cell, cell, color),

                8 => draw_rectangle(xy.0,                xy.1 + (cell * 2.0), cell, cell, color),
                9 => draw_rectangle(xy.0 + cell,         xy.1 + (cell * 2.0), cell, cell, color),
                10 => draw_rectangle(xy.0 + cell * 2.0,  xy.1 + (cell * 2.0), cell, cell, color),
                11 => draw_rectangle(xy.0 + cell * 3.0,  xy.1 + (cell * 2.0), cell, cell, color),

                12 => draw_rectangle(xy.0,                xy.1 + (cell * 3.0), cell, cell, color),
                13 => draw_rectangle(xy.0 + cell,         xy.1 + (cell * 3.0), cell, cell, color),
                14 => draw_rectangle(xy.0 + cell * 2.0,   xy.1 + (cell * 3.0), cell, cell, color),
                15 => draw_rectangle(xy.0 + cell * 3.0,   xy.1 + (cell * 3.0), cell, cell, color),
                _ => {}
            }

            // Grid vertical
            draw_line(xy.0 + cell, xy.1, xy.0 + cell, xy.1 + cell * 4.0, 2.0, BLACK);
            draw_line(xy.0 + cell * 2.0, xy.1, xy.0 + cell * 2.0, xy.1 + cell * 4.0, 2.0, BLACK);
            draw_line(xy.0 + cell * 3.0, xy.1, xy.0 + cell * 3.0, xy.1 + cell * 4.0, 2.0, BLACK);

            // Grid horizontal
            draw_line(xy.0, xy.1 + cell, xy.0 + cell * 4.0, xy.1 + cell, 2.0, BLACK);
            draw_line(xy.0, xy.1 + cell * 2.0, xy.0 + cell * 4.0, xy.1 + cell * 2.0, 2.0, BLACK);
            draw_line(xy.0, xy.1 + cell * 3.0, xy.0 + cell * 4.0, xy.1 + cell * 3.0, 2.0, BLACK);
        }
    }
}

impl Default for Pattern {
    fn default() -> Pattern {
        return Pattern {
            score: 0,
            multiplier: 0,
            scale: 1.0,
            display_info: false,
            tiles: [false; 16],
        };
    }
}