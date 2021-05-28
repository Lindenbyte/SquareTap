use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use macroquad::prelude::*;

const SCALE_MIN: f32 = 0.5;
const SCALE_MAX: f32 = 3.0;
const SCALE_CHANGE: f32 = 0.25;

pub struct Pattern {
    pub score: u32,
    pub multiplier: u16,
    pub scale: f32,
    time: f32,
    pub done: bool,
    clicks: u64,
    last_click_pos: (f32, f32),
    last_time_clicked: f64,
    display_info: bool,
    display_grid: bool,
    tiles: [bool; 16],
    tiles_size: f32,
    pub tiles_filled_color: Color,
    pub tiles_empty_color: Color,
    pub tiles_border_color: Color,
    pub crosshair_color: Color,
    pub score_color: Color,
    rng: ThreadRng,
}

impl Pattern {
    pub fn new() -> Self {
        // CHANGE: this later to all be default values
        let mut p = Self {
            ..Default::default()
        };
        p.setup();

        return p;
    }

    pub fn setup(&mut self) {
        self.score = 0;
        self.multiplier = 1;
        self.time = 30.0;
        self.clicks = 0;
        self.last_click_pos = (0.0, 0.0);
        self.last_time_clicked = 0.0;
        self.done = false;
        self.tiles = [false; 16];

        let mut generated_cells = 0;
        while generated_cells < 3 {
            let new_cell_pos = self.rng.gen_range(0..15);
            let new_cell_val = self.tiles[new_cell_pos];

            if !new_cell_val {
                self.tiles[new_cell_pos] = true;
                generated_cells += 1;
            }
        }
    }

    pub fn update(&mut self) {
        let current_time = get_time();

        if is_key_pressed(KeyCode::R) {
            self.setup();
        }

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
        self.tiles_size = 250.0 * self.scale;

        if !self.done {
            // Game time
            if self.time > 0.0 {
                self.time = self.time - get_frame_time();

                let dt_click = current_time - self.last_time_clicked;
                self.multiplier = (1.0 + (dt_click * (self.clicks as f64 / 4.5))).floor() as u16;
            } else {
                self.time = 0.0;
                self.done = true;
            }

            // Tiles
            let xy = (
                (screen_width() / 2.0) - self.tiles_size / 2.0,
                (screen_height() / 2.0) - self.tiles_size / 2.0,
            );
            let cell = self.tiles_size / 4.0;

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if (mouse_pos.0 > xy.0 && mouse_pos.1 > xy.1)
                    && (mouse_pos.0 < xy.0 + cell * 4.0 && mouse_pos.1 < xy.1 + cell * 4.0)
                {
                    let x = ((mouse_pos.0 - xy.0) / self.tiles_size * 4.0).floor();
                    let y = ((mouse_pos.1 - xy.1) / self.tiles_size * 4.0).floor();
                    let cell_pos = (x + (y * 4.0).floor()) as usize;

                    let cell_val = self.tiles[cell_pos];
                    if cell_val {
                        let mut new_cell_found = false;
                        while !new_cell_found {
                            let new_cell = self.rng.gen_range(0..15);
                            let new_cell_val = self.tiles[new_cell];

                            if !new_cell_val {
                                self.tiles[new_cell] = !new_cell_val;
                                new_cell_found = true;
                            }
                        }

                        self.tiles[cell_pos] = !self.tiles[cell_pos];
                        self.score += self.multiplier as u32;
                        self.last_time_clicked = current_time;
                        self.clicks += 1;
                    } else {
                        self.last_click_pos = mouse_pos;
                        self.done = true;
                    }
                }
            }
        }
    }

    pub fn render(&mut self, font: Font) {
        let xy = (
            (screen_width() / 2.0) - self.tiles_size / 2.0,
            (screen_height() / 2.0) - self.tiles_size / 2.0,
        );
        let cell = self.tiles_size / 4.0;

        // display_info information
        if self.display_info {
            draw_text(&*format!("Score: {}", self.score), 50.0, 70.0, 32.0, WHITE);
            draw_text(
                &*format!("Multiplier: {}", self.multiplier),
                50.0,
                100.0,
                32.0,
                WHITE,
            );
            draw_text(&*format!("Scale: {}", self.scale), 50.0, 130.0, 32.0, WHITE);
            draw_text(
                &*format!("Time: {}", self.time.floor()),
                50.0,
                160.0,
                32.0,
                WHITE,
            );
        }

        // Tile background. IE "outer grid lines"
        if self.display_grid {
            draw_rectangle(
                xy.0 - 2.0,
                xy.1 - 2.0,
                self.tiles_size + 4.0,
                self.tiles_size + 4.0,
                self.tiles_border_color,
            );
        }

        // Tiles and grid
        for tile in 0..16 {
            let filled: bool = self.tiles[tile];

            let color: Color;
            if filled {
                color = self.tiles_filled_color;
            } else {
                color = self.tiles_empty_color;
            }

            // Tiles
            match tile {
                0 => draw_rectangle(xy.0, xy.1, cell, cell, color),
                1 => draw_rectangle(xy.0 + cell, xy.1, cell, cell, color),
                2 => draw_rectangle(xy.0 + cell * 2.0, xy.1, cell, cell, color),
                3 => draw_rectangle(xy.0 + cell * 3.0, xy.1, cell, cell, color),

                4 => draw_rectangle(xy.0, xy.1 + cell, cell, cell, color),
                5 => draw_rectangle(xy.0 + cell, xy.1 + cell, cell, cell, color),
                6 => draw_rectangle(xy.0 + cell * 2.0, xy.1 + cell, cell, cell, color),
                7 => draw_rectangle(xy.0 + cell * 3.0, xy.1 + cell, cell, cell, color),

                8 => draw_rectangle(xy.0, xy.1 + (cell * 2.0), cell, cell, color),
                9 => draw_rectangle(xy.0 + cell, xy.1 + (cell * 2.0), cell, cell, color),
                10 => draw_rectangle(xy.0 + cell * 2.0, xy.1 + (cell * 2.0), cell, cell, color),
                11 => draw_rectangle(xy.0 + cell * 3.0, xy.1 + (cell * 2.0), cell, cell, color),

                12 => draw_rectangle(xy.0, xy.1 + (cell * 3.0), cell, cell, color),
                13 => draw_rectangle(xy.0 + cell, xy.1 + (cell * 3.0), cell, cell, color),
                14 => draw_rectangle(xy.0 + cell * 2.0, xy.1 + (cell * 3.0), cell, cell, color),
                15 => draw_rectangle(xy.0 + cell * 3.0, xy.1 + (cell * 3.0), cell, cell, color),
                _ => {}
            }

            if self.display_grid {
                // Grid vertical
                draw_line(
                    xy.0 + cell,
                    xy.1,
                    xy.0 + cell,
                    xy.1 + cell * 4.0,
                    2.0,
                    self.tiles_border_color,
                );
                draw_line(
                    xy.0 + cell * 2.0,
                    xy.1,
                    xy.0 + cell * 2.0,
                    xy.1 + cell * 4.0,
                    2.0,
                    self.tiles_border_color,
                );
                draw_line(
                    xy.0 + cell * 3.0,
                    xy.1,
                    xy.0 + cell * 3.0,
                    xy.1 + cell * 4.0,
                    2.0,
                    self.tiles_border_color,
                );

                // Grid horizontal
                draw_line(
                    xy.0,
                    xy.1 + cell,
                    xy.0 + cell * 4.0,
                    xy.1 + cell,
                    2.0,
                    self.tiles_border_color,
                );
                draw_line(
                    xy.0,
                    xy.1 + cell * 2.0,
                    xy.0 + cell * 4.0,
                    xy.1 + cell * 2.0,
                    2.0,
                    self.tiles_border_color,
                );
                draw_line(
                    xy.0,
                    xy.1 + cell * 3.0,
                    xy.0 + cell * 4.0,
                    xy.1 + cell * 3.0,
                    2.0,
                    self.tiles_border_color,
                );
            }
        }

        if self.done {
            let score_text = &*format!("SCORE: {}!!!", self.score);
            let score_text_dim = measure_text(score_text, Some(font), 64, 1.0);
            let color = self.score_color;
            draw_text_ex(
                score_text,
                screen_width() / 2.0 - score_text_dim.width / 2.0,
                (screen_height() / 2.0) - (self.tiles_size / 2.0) - 50.0,
                TextParams {
                    font,
                    font_size: 64,
                    color,
                    ..Default::default()
                },
            );

            // Last clicked position
            draw_circle(
                self.last_click_pos.0,
                self.last_click_pos.1,
                4.0,
                self.crosshair_color,
            );
        }

        // Cursor/Pointer
        let mouse_pos = mouse_position();
        draw_line(
            mouse_pos.0,
            mouse_pos.1 - 5.0,
            mouse_pos.0,
            mouse_pos.1 + 5.0,
            2.0,
            self.crosshair_color,
        );
        draw_line(
            mouse_pos.0 - 5.0,
            mouse_pos.1,
            mouse_pos.0 + 5.0,
            mouse_pos.1,
            2.0,
            self.crosshair_color,
        );
    }
}

impl Default for Pattern {
    fn default() -> Pattern {
        return Pattern {
            score: 0,
            multiplier: 1,
            scale: 1.0,
            time: 30.0,
            done: false,
            clicks: 0,
            last_click_pos: (0.0, 0.0),
            last_time_clicked: 0.0,
            display_info: true,
            display_grid: true,
            tiles: [false; 16],
            tiles_size: 250.0,
            tiles_filled_color: WHITE,
            tiles_empty_color: WHITE,
            tiles_border_color: WHITE,
            crosshair_color: WHITE,
            score_color: WHITE,
            rng: thread_rng(),
        };
    }
}
