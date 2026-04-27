use ratatui::crossterm::event::KeyCode;

use crate::random::Random;

pub struct App<'a>{
    rando: Random,
    pub num: u32,
    pub state: AppState,
    pub title: &'a str,
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
            is_exit: false,
        }
    }
    pub fn update(&mut self) {
        self.num = self.rando.rand();
    }
    pub fn input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.is_exit = true;
            }
            _ => {
                self.update();
            }
        }
    }
}