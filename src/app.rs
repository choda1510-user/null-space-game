use std::collections::HashMap;

use null_space::{GameCalender, e_str_to_year};
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

pub struct Game {
    pub random: Random,
    pub time: GameCalender,
    pub state: GameState,
    pub time_per_millis: GameCalender,
    pub ended_time: GameCalender,
}
pub enum GameState {
    Infomation,
    Production,
    PowerSavingMode,
}
impl Game {
    pub fn new() -> Game {
        Game {
            random: Random::new_default(),
            time: GameCalender::new(),
            state: GameState::Infomation,
            time_per_millis: GameCalender::from_time(1),
            ended_time: GameCalender::from_years(e_str_to_year("1.e12").unwrap()),
        }
    }
    pub fn new_seed(seed: u32) -> Game {
        Game {
            random: Random::new(seed),
            time: GameCalender::new(),
            state: GameState::Infomation,
            time_per_millis: GameCalender::from_time(1),
            ended_time: GameCalender::from_years(e_str_to_year("1.e12").unwrap()),
        }
    }
    pub fn update(&mut self, millis: u128) {
        for _ in 0..millis {
            self.time.add_calender(&self.time_per_millis);
        }
    }
    pub fn time_speed_faster(&mut self) {
        self.time_per_millis.add_calender(&self.time_per_millis.clone());
    }
    pub fn time_speed_reset(&mut self) {
        self.time_per_millis = GameCalender::from_time(1);
    }
    pub fn time_stop(&mut self) {
        self.time_per_millis = GameCalender::new();
    }
}
pub struct SpaceShip {
    power: u32,
    speed: f64,
    items: HashMap<Item, u64>, // item, number
    max_items_volumn: u64,
    resources: HashMap<Resource, f64>, // resource, amount
    max_resources_amount: f64,
    crafting_slot: Vec<Vec<Recipe>>,
    max_crafting_slot: usize,
}
pub struct Item {
    code: i32,
    name: String,
    weight: f64,
    volumn: u64,
}
pub struct Resource {
    code: i32,
    name: String,
    weight: f64, // 1.0 per weight
}
fn create_resources() {
    // 1.
    let hydrogen = Resource {
        code: 1,
        name: String::from("hydrogen"),
        weight: 1.008,
    };
    let helium = Resource {
        code: 2,
        name: String::from("helium"),
        weight: 4.0026,
    };

    // 2.

    let lightweight_dust = Resource {
        code: 379,
        name: String::from("lightweight-dust"),
        weight: 1.0,
    };
    let dust = Resource {
        code: 380,
        name: String::from("dust"),
        weight: 2.0,
    };
    let heavy_dust = Resource {
        code: 381,
        name: String::from("heavy-dust"),
        weight: 8.0,
    };
    let fuel = Resource {
        code: 382,
        name: String::from("fuel"),
        weight: 0.98,
    };

}
pub struct Recipe {
    code: i32,
    name: String,
    input_items: HashMap<Item, u64>,
    input_resources: HashMap<Resource, f64>,
    spend_time: GameCalender,
    output_items: HashMap<Item, u64>,
    output_resources: HashMap<Resource, f64>,
}
pub struct Blackhole {
    ended_time: GameCalender,
    escape_speed: f64,
    accretion_disk: HashMap<Resource, f64>,
}