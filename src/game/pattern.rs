use macroquad::prelude::*;

const SCALE_MIN: f32 = 0.5;
const SCALE_MAX: f32 = 3.0;
const SCALE_CHANGE: f32 = 0.25;

const TILE_FILLED_COLOR: Color = color_u8!(40, 130, 115, 255);
const TILE_EMPTY_COLOR: Color = color_u8!(45, 55, 65, 255);
const TILE_BORDER_COLOR: Color = color_u8!(255, 255, 255, 255);

pub struct Pattern {
    pub score: u32,
    pub multiplier: u16,
    scale: f32,
    display_info: bool,
    display_grid: bool,
    tiles: [bool; 16],
    tiles_size: f32
}

impl Pattern {
    pub fn new() -> Self {
        // CHANGE: this later to all be default values
        // return Self { ..Default::default() };
        return Self { tiles: [false, true, false, true, true, false, true, false, false, true, false, true, true, false, true, false], ..Default::default() };
    }

    pub fn update(&mut self) {
        self.tiles_size = 250.0 * self.scale;

        if is_key_pressed(KeyCode::Tab) {
            self.display_info = !self.display_info;
        }

        if is_key_pressed(KeyCode::G) {
            self.display_grid = !self.display_grid;
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
        
        let xy = (
            (screen_width() / 2.0) - self.tiles_size / 2.0, 
            (screen_height() / 2.0) - self.tiles_size / 2.0
        );
        let cell = self.tiles_size / 4.0;

        // display_info information
        if self.display_info {
            draw_text(
                &*format!("Score: {}", self.score), 
                50.0, 70.0, 32.0, WHITE
            );
            draw_text(
                &*format!("Multiplier: {}", self.multiplier), 
                50.0, 100.0, 32.0, WHITE
            );
            draw_text(
                &*format!("Scale: {}", self.scale), 
                50.0, 130.0, 32.0, WHITE
            );
        }

        // Tile background. IE "outer grid lines"
        if self.display_grid {
            draw_rectangle(xy.0 - 2.0, xy.1 - 2.0, self.tiles_size + 4.0, self.tiles_size + 4.0, TILE_BORDER_COLOR);
        }
        
        // Tiles and grid
        for tile in 0..16 {
            let filled: bool = self.tiles[tile];

            let color: Color;
            if filled {
                color = TILE_FILLED_COLOR;
            } else {
                color = TILE_EMPTY_COLOR;
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

            if self.display_grid {
                // Grid vertical
                draw_line(xy.0 + cell, xy.1, xy.0 + cell, xy.1 + cell * 4.0, 2.0, TILE_BORDER_COLOR);
                draw_line(xy.0 + cell * 2.0, xy.1, xy.0 + cell * 2.0, xy.1 + cell * 4.0, 2.0, TILE_BORDER_COLOR);
                draw_line(xy.0 + cell * 3.0, xy.1, xy.0 + cell * 3.0, xy.1 + cell * 4.0, 2.0, TILE_BORDER_COLOR);
    
                // Grid horizontal
                draw_line(xy.0, xy.1 + cell, xy.0 + cell * 4.0, xy.1 + cell, 2.0, TILE_BORDER_COLOR);
                draw_line(xy.0, xy.1 + cell * 2.0, xy.0 + cell * 4.0, xy.1 + cell * 2.0, 2.0, TILE_BORDER_COLOR);
                draw_line(xy.0, xy.1 + cell * 3.0, xy.0 + cell * 4.0, xy.1 + cell * 3.0, 2.0, TILE_BORDER_COLOR);
            }
        }
    }
}

impl Default for Pattern {
    fn default() -> Pattern {
        return Pattern {
            score: 0,
            multiplier: 0,
            scale: 1.0,
            display_info: true,
            display_grid: true,
            tiles: [false; 16],
            tiles_size: 250.0
        };
    }
}