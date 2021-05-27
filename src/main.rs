use macroquad::prelude::*;

mod game;
use game::Game;

fn window_config() -> Conf {
    return Conf {
        window_title: "SquareTap".to_owned(),
        fullscreen: true,
        high_dpi: true,
        window_resizable:  false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new();
    game.setup().await;
   
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width() as f32, screen_height() as f32));
    set_camera(&camera);

    while game.state != game::GameState::Closing {
        game.update();
        game.render();
        
        next_frame().await;
    }
}