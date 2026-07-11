use ratatui::crossterm::event::KeyCode;

use crate::random::Random;
use crate::game::Game;

pub struct App<'a>{
    rando: Random,
    pub num: u32,
    pub state: AppState,
    pub title: &'a str,
    pub select_item: usize,
    pub select_items: Vec<&'a str>,
    pub is_exit: bool,
    pub game: Option<Game>,
}
pub enum AppState {
    MainScreen,
    Playing
}
impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App::new_random(Random::new_default())
    }
    pub fn new_by_seed(seed: u32) -> App<'a> {
        App::new_random(Random::new(seed))
    }
    fn new_random(random: Random) -> App<'a>{
        App {
            rando: random,
            num: 0,
            state: AppState::MainScreen,
            title: "Null Space",
            select_item: 0,
            select_items: vec!["start", "exit"],
            is_exit: false,
            game: None
        }
    }
    pub fn update(&mut self) {
        self.num = self.rando.rand();
    }
    pub fn input(&mut self, key: KeyCode) {
        match self.state {
            AppState::MainScreen => {
                match key {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.is_exit = true;
                    }
                    KeyCode::Up => {
                        if self.select_item > 0 {
                            self.select_item -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.select_item < 1 {
                            self.select_item += 1;
                        }
                    }
                    KeyCode::Enter => {
                        match self.select_item {
                            0 => {
                                self.state = AppState::Playing;
                                if let None = self.game {
                                    self.game = Some(Game::new())
                                }
                            }
                            1 => {
                                self.is_exit = true;
                            }
                            _ => {

                            }
                        }
                    }
                    _ => {
                    }
                }
            }
            AppState::Playing => {
                match key {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.state = AppState::MainScreen;
                        self.game = None;
                    }
                    KeyCode::Char('e') => {
                        if let Some(game) = &mut self.game {
                            game.power_save_toggle();
                        }
                    }
                    KeyCode::Char('+') => {
                        if let Some(game) = &mut self.game {
                            game.time_speed_faster();
                        }
                    }
                    KeyCode::Char('-') => {
                        if let Some(game) = &mut self.game {
                            game.time_speed_reset();
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(game) = &mut self.game {
                            game.time_stop();
                        }
                    }
                    _ => {
                        self.update();
                    }
                }
            }
        }
    }
}
