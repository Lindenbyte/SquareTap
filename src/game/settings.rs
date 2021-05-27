use macroquad::prelude::*;

pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
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
