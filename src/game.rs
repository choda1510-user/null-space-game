use std::collections::HashMap;
use null_space::{
    GameCalender,
    e_str_to_year
};
use crate::random::Random;

pub struct Game {
    pub random: Random,
    pub time: GameCalender,
    pub state: GameState,
    pub time_per_millis: GameCalender,
    time_per_millis_prev: GameCalender,
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
            time_per_millis_prev: GameCalender::new(),
            ended_time: GameCalender::from_years(e_str_to_year("1.e12").unwrap()),
        }
    }
    pub fn new_seed(seed: u32) -> Game {
        Game {
            random: Random::new(seed),
            time: GameCalender::new(),
            state: GameState::Infomation,
            time_per_millis: GameCalender::from_time(1),
            time_per_millis_prev: GameCalender::new(),
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
    pub fn power_save_toggle(&mut self) {
        match self.state {
            GameState::PowerSavingMode => {
                self.state = GameState::Infomation;
                self.time_per_millis = GameCalender::from_time(1);
            }
            _ => {
                self.state = GameState::PowerSavingMode;
                self.time_per_millis = GameCalender::from_years(vec![1]);
            }
        }
    }
    pub fn time_speed_reset(&mut self) {
        self.time_per_millis = GameCalender::from_time(1);
    }
    pub fn time_stop(&mut self) {
        let zero: GameCalender = GameCalender::new();
        if self.time_per_millis == zero {
            self.time_per_millis = self.time_per_millis_prev.clone();
        } else {
            self.time_per_millis_prev = self.time_per_millis.clone();
            self.time_per_millis = zero;
        }
    }
}
pub struct SpaceShip {
    power: u16,
    speed: f64,
    items: HashMap<i32, u64>, // item, number
    max_items_volumn: u64,
    resources: HashMap<i32, f64>, // resource, amount
    max_resources_amount: f64,
    crafting_slot: Vec<Vec<CraftingTask>>,
    max_crafting_slot: usize,
}
pub struct Item {
    code: i32,
    name: String,
    weight: f64,
    volumn: u64,
}
fn _create_items() {
    let _iron_ore = Item {
        code: 1,
        name: String::from("iron-ore"),
        weight: 3.0,
        volumn: 5000
    };
    let _gold_ore = Item {
        code: 2,
        name: String::from("gold-ore"),
        weight: 3.2,
        volumn: 5000
    };
    let _component = Item {
        code: 100,
        name: String::from("component"),
        weight: 0.25,
        volumn: 1000
    };
}
pub struct Resource {
    code: i32,
    name: String,
    weight: f64, // 1.0 per weight
}
fn _create_resources() {
    // 1.
    let _hydrogen = Resource {
        code: 1,
        name: String::from("hydrogen"),
        weight: 1.008,
    };
    let _helium = Resource {
        code: 2,
        name: String::from("helium"),
        weight: 4.0026,
    };

    // 2.

    let _lightweight_dust = Resource {
        code: 379,
        name: String::from("lightweight-dust"),
        weight: 1.0,
    };
    let _dust = Resource {
        code: 380,
        name: String::from("dust"),
        weight: 2.0,
    };
    let _heavy_dust = Resource {
        code: 381,
        name: String::from("heavy-dust"),
        weight: 8.0,
    };
    let _fuel = Resource {
        code: 382,
        name: String::from("fuel"),
        weight: 0.98,
    };

}
pub struct Recipe {
    code: i32,
    name: String,
    input_items: HashMap<i32, u64>,
    input_resources: HashMap<i32, f64>,
    spend_time: GameCalender,
    output_items: HashMap<i32, u64>,
    output_resources: HashMap<i32, f64>,
}
pub struct GameRegistry {
    item_list: HashMap<i32, Item>,
    resource_list: HashMap<i32, Resource>,
    recipe_list: HashMap<i32, Recipe>,
}
pub enum GameRegistryError {
    Duplicate,
    NotFound,
    Reference,
}
impl GameRegistry {
    fn new() -> GameRegistry {
        GameRegistry {
            item_list: HashMap::new(),
            resource_list: HashMap::new(),
            recipe_list: HashMap::new(),
        }
    }
    fn new_by(item_list: HashMap<i32, Item>, resource_list: HashMap<i32, Resource>, recipe_list: HashMap<i32, Recipe>) -> GameRegistry {
        GameRegistry {
            item_list: item_list,
            resource_list: resource_list,
            recipe_list: recipe_list,
        }
    }
    fn add_item(&mut self, item: Item) -> Result<&mut Self, GameRegistryError> {
        match self.item_list.insert(item.code, item) {
            None => {
                Ok(self)
            }
            Some(prev_item) => {
                self.item_list.remove(&prev_item.code);
                match self.item_list.insert(prev_item.code, prev_item) {
                    None => {
                        Err(GameRegistryError::Duplicate)
                    }
                    Some(_) => {
                        panic!("unexpected result")
                    }
                }
            }
        }
    }
    fn get_item(&self, id: i32) -> Option<&Item> {
        self.item_list.get(&id)
    }
    fn remove_item(&mut self, id: i32) -> Option<Item> {
        let recipes = self.recipe_list.values();
        for recipe in recipes {
            if (*recipe).input_items.contains_key(&id) {
                return None;
            }
            if (*recipe).output_items.contains_key(&id) {
                return None;
            }
        }
        self.item_list.remove(&id)
    }
    fn add_resource(&mut self, resource: Resource) -> Result<&mut Self, GameRegistryError> {
        match self.resource_list.insert(resource.code, resource) {
            None => {
                Ok(self)
            }
            Some(prev_resource) => {
                self.resource_list.remove(&prev_resource.code);
                match self.resource_list.insert(prev_resource.code, prev_resource) {
                    None => {
                        Err(GameRegistryError::Duplicate)
                    }
                    Some(_) => {
                        panic!("unexpected result")
                    }
                }
            }
        }
    }
    fn get_resource(&self, id: i32) -> Option<&Resource> {
        self.resource_list.get(&id)
    }
    fn remove_resource(&mut self, id: i32) -> Option<Resource> {
        let recipes = self.recipe_list.values();
        for recipe in recipes {
            if (*recipe).input_resources.contains_key(&id) {
                return None;
            }
            if (*recipe).output_resources.contains_key(&id) {
                return None;
            }
        }
        self.resource_list.remove(&id)
    }
    fn add_recipe(&mut self, recipe: Recipe) -> Result<&mut Self, GameRegistryError> {
        for &key in recipe.input_items.keys() {
            if let None = self.get_item(key) {
                return Err(GameRegistryError::NotFound);
            }
        }
        for &key in recipe.input_resources.keys() {
            if let None = self.get_resource(key) {
                return Err(GameRegistryError::NotFound);
            }
        }
        for &key in recipe.output_items.keys() {
            if let None = self.get_item(key) {
                return Err(GameRegistryError::NotFound);
            }
        }
        for &key in recipe.output_resources.keys() {
            if let None = self.get_item(key) {
                return Err(GameRegistryError::NotFound);
            }
        }
        match self.recipe_list.insert(recipe.code, recipe) {
            None => {
                Ok(self)
            }
            Some(prev_recipe) => {
                self.recipe_list.remove(&prev_recipe.code);
                match self.recipe_list.insert(prev_recipe.code, prev_recipe) {
                    None => {
                        Err(GameRegistryError::Duplicate)
                    }
                    Some(_) => {
                        panic!("unexpected result")
                    }
                }
            }
        }
    }
    fn get_recipe(&self, id: i32) -> Option<&Recipe> {
        self.recipe_list.get(&id)
    }
    fn remove_recipe(&mut self, id: i32) -> Option<Recipe> {
        self.recipe_list.remove(&id)
    }
}
pub struct CraftingTask {
    start_time: Option<GameCalender>,
    recipe: i32,
    process: u16,
}
pub struct Blackhole {
    ended_time: GameCalender,
    escape_speed: f64,
    accretion_disk: HashMap<i32, f64>,
}