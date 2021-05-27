use macroquad::prelude::*;

pub struct Highscore {
    scores: [u32; 9],
}

impl Highscore {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    pub fn setup(&mut self) {}

    pub fn score_exist(&mut self, score: u32) -> bool {
        return self.scores.iter().find(|&&val| val == score) == Some(&score);
    }

    pub fn add_score(&mut self, score: u32) {
        let mut last_changed_score = score;
        for i in 0..9 {
            if last_changed_score > self.scores[i] {
                let temp = self.scores[i];
                self.scores[i] = last_changed_score;
                last_changed_score = temp;
            } else if last_changed_score == self.scores[i] {
                return;
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, font: Font) {
        let title = "Highscore";
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

        let xy: (f32, f32) = (250.0, screen_height() / 2.0 - 150.0);
        for i in 0..9 {
            draw_text_ex(
                &*format!("{}:{: >10}", i + 1, self.scores[i]),
                xy.0,
                xy.1 + (i as f32 * 50.0),
                TextParams {
                    font,
                    font_size: 32,
                    ..Default::default()
                },
            );
        }
    }
}

impl Default for Highscore {
    fn default() -> Highscore {
        return Highscore { scores: [0; 9] };
    }
}
