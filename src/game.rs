use winapi::um::winuser::ShowCursor;
use macroquad::prelude::*;

mod pattern;
use pattern::Pattern;

#[derive(PartialEq)]
pub enum GameState {
    Menu,
    Running,
    Highscore,
    Closing
}

#[derive(PartialEq)]
enum MenuSelect {
    Run,
    Highscore,
    Close
}

pub struct Game {
    pub state: GameState,
    menu_selected: MenuSelect,
    menu_background: Texture2D,    
    font: Font,

    pattern: Pattern,
}

impl Game {
    pub fn new() -> Self {
        return Self { ..Default::default() }
    }

    pub async fn setup(&mut self) {
        unsafe {
            ShowCursor(0);
        }

        self.menu_background = load_texture("res/img/menu_background.png").await.unwrap();
        self.font = load_ttf_font("res/fonts/alagard.ttf").await.unwrap();
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Up) {
                    match self.menu_selected {
                        MenuSelect::Run => {},
                        MenuSelect::Highscore => self.menu_selected = MenuSelect::Run,
                        MenuSelect::Close => self.menu_selected = MenuSelect::Highscore
                    }
                } else if is_key_pressed(KeyCode::Down) {
                    match self.menu_selected {
                        MenuSelect::Run => self.menu_selected = MenuSelect::Highscore,
                        MenuSelect::Highscore => self.menu_selected = MenuSelect::Close,
                        MenuSelect::Close => {}
                    }
                }

                if is_key_pressed(KeyCode::Enter) {
                    match self.menu_selected {
                        MenuSelect::Run => {
                            self.pattern.setup();
                            self.state = GameState::Running;
                        },
                        MenuSelect::Highscore => {
                            self.state = GameState::Highscore;
                        },
                        MenuSelect::Close => {
                            self.state = GameState::Closing;
                        }
                    }
                }
            },
            GameState::Running => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
                
                self.pattern.update();
            },
            GameState::Highscore => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            },
            GameState::Closing => {}
        }
    }

    pub fn render(&mut self) {
        let font = self.font;
        
        clear_background(BLACK);
        // Background
        draw_texture(self.menu_background, 0.0, 0.0, WHITE);

        match self.state {
            GameState::Menu => {
                let title = "SquareTap v0.1.1 - alpha";
                let title_dimensions = measure_text(title, Some(font), 78, 1.0);
                draw_text_ex(title, 
                    screen_width() / 2.0 - title_dimensions.width / 2.0, screen_height() / 2.0 - 250.0, 
                    TextParams{ font, font_size: 78, color: WHITE, ..Default::default()}
                );

                let selected = Color::from_rgba(255, 255, 255, 255);
                let not_selected = Color::from_rgba(255, 255, 255, 125);
                let mut start_color = not_selected;
                let mut highscore_color = not_selected;
                let mut close_color = not_selected;
                match self.menu_selected {
                    MenuSelect::Run => start_color = selected,
                    MenuSelect::Highscore => highscore_color = selected,
                    MenuSelect::Close => close_color = selected
                }

                draw_text_ex("Start", 
                    250.0, screen_height()/2.0 - 50.0, 
                    TextParams{ font, font_size: 32, color: start_color, ..Default::default() }
                );

                draw_text_ex("Highscore", 
                    250.0, screen_height()/2.0, 
                    TextParams{ font, font_size: 32, color: highscore_color, ..Default::default() }
                );

                draw_text_ex("Exit", 
                    250.0, screen_height()/2.0 + 50.0, 
                    TextParams{ font, font_size: 32, color: close_color, ..Default::default() }
                );
            },
            GameState::Running => self.pattern.render(font),
            GameState::Highscore => {},
            GameState::Closing => {}
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        return Game {
            state: GameState::Menu,
            menu_selected: MenuSelect::Run,
            menu_background: Texture2D::empty(),
            font: Font::default(),
            pattern: Pattern::new(),
        }
    }
}