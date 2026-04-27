use std::time::SystemTime;

pub struct Random {
    seed: u32
}
impl Random {
    pub fn new(seed: u32) -> Random {
        Random {
            seed: seed
        }
    }
    pub fn new_default() -> Random {
        let now = SystemTime::now();
        Random {
            seed: (now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()) as u32
        }
    }
    pub fn rand(&mut self) -> u32 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x << 17;
        x ^= x << 5;
        self.seed = x;
        return x;
    }
}