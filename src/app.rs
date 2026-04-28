use ratatui::crossterm::event::KeyCode;

use crate::random::Random;

pub struct App<'a>{
    rando: Random,
    pub num: u32,
    pub state: AppState,
    pub title: &'a str,
    pub select_item: usize,
    pub select_items: Vec<&'a str>,
    pub is_exit: bool,
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
                    }
                    _ => {
                        self.update();
                    }
                }
            }
        }
    }
}