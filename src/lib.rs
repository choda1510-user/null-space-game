use std::ops::Add;
const YEAR: [u8; 8] = [0,0x60,0xf1,0x3d,0x07,0,0,0];
pub struct GameCalender {
    // pub year_exponent: u64,
    // pub year_significand: u64,
    // pub mouth: u32,
    // pub day: u32,
    // pub hour: u32,
    // pub minute: u32,
    // pub second: u32,
    // pub millis: u32,
    // pub micros: u32,
    // pub nanos: u32,
    tick: [u8;16*4],
    // year: [u32; 16],
    // time: u64,
}
impl GameCalender {
    pub fn new() -> GameCalender {
        GameCalender {
            tick: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        }
    }
    pub fn millis(&self) -> u64 {
        self.moduler(1000)
    }
    fn moduler(&self, period: u64) -> u64 {
        let mut result = self.tick[0] as u64;
        for i in 1.. self.tick.len() {
            let mut exponent = 256;
            for _ in 1..i {
                exponent = (exponent % period) * (256 % period) % period;
            }
            let temp = (self.tick[i] as u64 % period) * (exponent % period)  % period;
            result = (result % period + temp % period) % period;
        }
        result
    }
    pub fn add_millis(&mut self, millis: u64) {
        self.add_tick(millis.to_le_bytes().to_vec());
    }
    pub fn second(&self) -> u64 {
        self.moduler(60 * 1000) / 1000
    }
    pub fn add_second(&mut self, second: u64) {
        self.add_millis(second * 1000);
    }
    pub fn minute(&self) -> u64 {
        self.moduler(60 * 60 * 1000) / (60 * 1000)
    }
    pub fn add_minute(&mut self, minute: u64) {
        self.add_second(minute * 60);
    }
    pub fn hour(&self) -> u64 {
        self.moduler(24 * 60 * 60 * 1000) / (60 * 60 * 1000)
    }
    pub fn add_hour(&mut self, hour: u64) {
        self.add_minute(hour * 60);
    }
    pub fn day(&self) -> u64 {
        self.moduler(30 * 24 * 60 * 60 * 1000) / (24 * 60 * 60 * 1000)
    }
    pub fn add_day(&mut self, day: u64) {
        self.add_hour(day * 24);
    }
    pub fn month(&self) -> u64 {
        self.moduler(12 * 30 * 24 * 60 * 60 * 1000) / (30 * 24 * 60 * 60 * 1000)
    }
    pub fn add_month(&mut self, month: u64) {
        self.add_day(month * 30);
    }
    pub fn get_ticks(&self) -> &[u8; 16*4]{
        &self.tick
    }
    pub fn increase_year(&mut self) {
        self.add_tick(YEAR.clone().to_vec());
    }
    pub fn add_tick(&mut self, tick: Vec<u8>) {
        let mut byte;
        let mut is_overflowed;
        let mut prev_overflowed = false;
        if tick.len() > self.tick.len() {
            panic!("error, because year is too big.");
        } else {
            for i in 0..tick.len() {
                (byte, is_overflowed) = self.tick[i].overflowing_add(tick[i]);
                if prev_overflowed {
                    if is_overflowed {
                        byte = byte + 1;
                    } else {
                        (byte, is_overflowed) = byte.overflowing_add(1);
                    }
                    prev_overflowed = false;
                }
                if is_overflowed {
                    prev_overflowed = true;
                }
                self.tick[i] = byte;
            }
            is_overflowed = prev_overflowed;
            for i in tick.len()..self.tick.len() {
                if is_overflowed {
                    (byte, is_overflowed) = self.tick[i].overflowing_add(1);
                    self.tick[i] = byte;
                } else {
                    break;
                }
            }
        }
    }
}
pub fn year_to_str(cal: &GameCalender) -> String{
    let ticks = cal.tick.clone().to_vec();
    let mut tick_count: Vec<u8> = Vec::new();
    for _ in 0..ticks.len() {
        tick_count.push(0);
    }
    let year_period = 12 as u64 * 30 as u64 * 24 as u64 * 60 as u64 * 60 as u64 * 1000 as u64;
    let year_arr = year_period.to_le_bytes().to_vec();
    let mut year_cells = vec![];
    loop {
        add_bytes(&mut tick_count, &year_arr);
        if left_small_than_right(&tick_count, &ticks) {
            add_year_cells(&mut year_cells);
        } else {
            break match year_cells.iter()
                .map(|&num| num.to_string())
                .rev()
                .reduce(|str1, str2| str1.add(&str2)) {
                    Some(result) => result,
                    None => String::from("0")
                }
        }
    }
}
fn add_year_cells(year_cells: &mut Vec<u8>) {
    if year_cells.len() == 0 {
        year_cells.push(1);
        return;
    }
    let mut is_overflowed = true;

    for i in 0..year_cells.len() {
        let cell = year_cells[i] + if is_overflowed { 1 } else { 0 };
        if cell >= 10 {
            year_cells[i] = 0;
            is_overflowed = true;
        } else {
            year_cells[i] += 1;
            is_overflowed = false;
            break;
        }
    }
    if is_overflowed {
        year_cells.push(1);
    }
}
fn add_bytes(left: &mut Vec<u8>, right: &Vec<u8>) {
    let mut byte;
    let mut is_overflowed;
    let mut prev_overflowed = false;
    for i in 0..right.len() {
        (byte, is_overflowed) = left[i].overflowing_add(right[i]);
        if prev_overflowed {
            if is_overflowed {
                byte = byte + 1;
            } else {
                (byte, is_overflowed) = byte.overflowing_add(1);
            }
            prev_overflowed = false;
        }
        if is_overflowed {
            prev_overflowed = true;
        }
        left[i] = byte;
    }
    is_overflowed = prev_overflowed;
    for i in right.len()..left.len() {
        if is_overflowed {
            (byte, is_overflowed) = left[i].overflowing_add(1);
            left[i] = byte;
        } else {
            break;
        }
    }
}
fn left_small_than_right(left: &Vec<u8>, right: &Vec<u8>) -> bool {
    if left.len() > right.len() {
        return false;
    }
    let mut than = true;
    for i in 0..left.len() {
        if left[i] < right[i] {
            than = true;
        }
        if left[i] > right[i] {
            than = false;
        }
    }
    than
}
#[cfg(test)]
mod test {
    use crate::GameCalender;
    use crate::add_year_cells;
    use crate::left_small_than_right;
    use crate::year_to_str;

    #[test]
    fn add_tick_to_millis() {
        let mut cal = GameCalender::new();
        let init_millis = cal.millis();
        cal.add_millis(600);
        let added_millis = cal.millis();
        cal.add_millis(1800);
        let overflow_added_millis = cal.millis();

        assert_eq!(init_millis, 0);
        assert_eq!(added_millis, 600);
        assert_eq!(overflow_added_millis, 400);
    }
    #[test]
    fn add_tick_to_second() {
        let mut cal = GameCalender::new();
        let init_second = cal.second();
        cal.add_millis(600);
        let added_second = cal.second();
        cal.add_millis(1800);
        let show_added_second = cal.second();
        cal.add_millis(59000);
        let overflow_added_second = cal.second();

        assert_eq!(init_second, 0);
        assert_eq!(added_second, 0);
        assert_eq!(show_added_second, 2);
        assert_eq!(overflow_added_second, 1);
    }
    #[test]
    fn add_second_to_second() {
        let mut cal = GameCalender::new();
        let init_second = cal.second();
        cal.add_second(6);
        let added_second = cal.second();
        cal.add_second(57);
        let overflow_added_second = cal.second();

        assert_eq!(init_second, 0);
        assert_eq!(added_second, 6);
        assert_eq!(overflow_added_second, 3);
    }
    #[test]
    fn add_minute() {
        let mut cal = GameCalender::new();
        cal.add_minute(2);
        cal.add_second(34);
        cal.add_second(32);
        let add_minute = cal.minute();
        let add_second = cal.second();

        assert_eq!(add_minute, 3);
        assert_eq!(add_second, 6);
    }
    #[test]
    fn add_hour() {
        let mut cal = GameCalender::new();
        cal.add_hour(3);
        cal.add_minute(24);
        cal.add_minute(48);
        let add_hour = cal.hour();
        let add_minute = cal.minute();

        assert_eq!(add_hour, 4);
        assert_eq!(add_minute, 12);
    }
    #[test]
    fn add_day() {
        let mut cal = GameCalender::new();
        cal.add_day(5);
        cal.add_hour(13);
        cal.add_hour(18);
        let add_day = cal.day();
        let add_hour = cal.hour();

        assert_eq!(add_day, 6);
        assert_eq!(add_hour, 7);
    }
    #[test]
    fn add_month() {
        let mut cal = GameCalender::new();
        cal.add_month(2);
        cal.add_day(28);
        cal.add_day(3);
        let add_month = cal.month();
        let add_day = cal.day();

        assert_eq!(add_month, 3);
        assert_eq!(add_day, 1);

        cal.add_month(11);
        let overflow_added_month = cal.month();

        assert_eq!(overflow_added_month, 2);
    }
    #[test]
    fn left_small_than_right_test() {
        assert_eq!(left_small_than_right(&vec![0,0], &vec![0,1]), true);
        assert_eq!(left_small_than_right(&vec![0,0], &vec![1,0]), true);
        assert_eq!(left_small_than_right(&vec![0,1], &vec![1,1]), true);
        assert_eq!(left_small_than_right(&vec![0,1], &vec![0,0]), false);
        assert_eq!(left_small_than_right(&vec![0,0], &vec![0,0]), true);
        assert_eq!(left_small_than_right(&vec![0,1], &vec![0,0]), false);

        assert_eq!(left_small_than_right(&vec![0,255,0], &vec![0,1,1]), true);
        assert_eq!(left_small_than_right(&vec![0,0,3], &vec![1,0,3]), true);
        assert_eq!(left_small_than_right(&vec![4,0,10], &vec![0,1,10]), true);
        assert_eq!(left_small_than_right(&vec![0,1,1], &vec![0,255,0]), false);
        assert_eq!(left_small_than_right(&vec![1,0,3], &vec![0,0,3]), false);
        assert_eq!(left_small_than_right(&vec![0,1,10], &vec![4,0,10]), false);
    }
    #[test]
    fn add_year_cell_test() {
        let mut year_cells: Vec<u8> = vec![];
        assert_eq!(year_cells.len(), 0);
        add_year_cells(&mut year_cells);
        assert_eq!(year_cells[0], 1);
        add_year_cells(&mut year_cells);
        assert_eq!(year_cells[0], 2);
        for _ in 0..10 { add_year_cells(&mut year_cells); }
        assert_eq!(year_cells[1], 1); assert_eq!(year_cells[0], 2);
        for _ in 0..10 { add_year_cells(&mut year_cells); }
        add_year_cells(&mut year_cells);
        assert_eq!(year_cells[1], 2); assert_eq!(year_cells[0], 3);
    }
    #[test]
    fn zero_year_to_string() {
        let cal = GameCalender::new();
        let year = year_to_str(&cal);

        assert_eq!(year, "0");
    }
    #[test]
    fn add_one_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_tick(vec![0,0x60,0xf1,0x3d,0x07,0,0,0]);
        let year = year_to_str(&cal);

        assert_eq!(year, "1");
    }
    #[test]
    fn add_two_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_tick(vec![0,0x60,0xf1,0x3d,0x07,0,0,0]);
        cal.add_tick(vec![0,0x60,0xf1,0x3d,0x07,0,0,0]);
        let year = year_to_str(&cal);

        assert_eq!(year, "2");
    }
    #[test]
    fn add_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_tick(vec![0,0x60,0xf1,0x3d,0x07,0,0,0]);
        cal.add_tick(vec![0,0x60,0xf1,0x3d,0x07,0,0,0]);
        cal.add_month(7);
        cal.add_month(5);
        let year = year_to_str(&cal);
        let month = cal.month();

        assert_eq!(year, "3");
        assert_eq!(month, 0);
    }
}