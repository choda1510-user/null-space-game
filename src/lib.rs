use std::ops::Add;
pub struct GameCalender {
    year: Vec<u8>,
    time: u64,
}
impl GameCalender {
    pub fn new() -> GameCalender {
        GameCalender {
            year: Vec::from([0 as u8]),
            time: 0,
        }
    }
    pub fn millis(&self) -> u64 {
        self.time % 1000
    }
    fn add_time(&mut self, time: u64) {
        const OVERFLOW_YEAR: u64 = u64::MAX / (12 * 30 * 24 * 60 * 60 * 1000);
        const OVERFLOW_TIME: u64 = u64::MAX % (12 * 30 * 24 * 60 * 60 * 1000);
        let (result, is_overflowed) = self.time.overflowing_add(time);

        let d1 = result / (12 * 30 * 24 * 60 * 60 * 1000);
        let m1 = result % (12 * 30 * 24 * 60 * 60 * 1000);

        if is_overflowed {
            let d2 = d1 + OVERFLOW_YEAR;
            let m2 = m1 + OVERFLOW_TIME;
            let d3 = d2 + m2 / (12 * 30 * 24 * 60 * 60 * 1000);
            let m3 = m2 % (12 * 30 * 24 * 60 * 60 * 1000);
            self.add_years(u64_to_u8_arr(d3));
            self.time = m3;
        } else {
            self.add_years(u64_to_u8_arr(d1));
            self.time = m1;
        }
    }

    pub fn add_years(&mut self, years: Vec<u8>) {
        let mut num;
        let mut is_overflowed = false;
        if years.len() > self.year.len() {
            for _ in 0..(years.len()-self.year.len()) {
                self.year.push(0);
            }
        } else {
            for i in 0..years.len() {
                num = self.year[i] + years[i] + if is_overflowed { 1 } else { 0 };
                if num >= 10 {
                    num %= 10;
                    is_overflowed = true;
                } else {
                    is_overflowed = false;
                }
                self.year[i] = num;
            }
            for i in years.len()..self.year.len() {
                if is_overflowed {
                    num = self.year[i] + if is_overflowed { 1 } else { 0 };
                    if num >= 10 {
                        num %= 10;
                        self.year[i] = num;
                        is_overflowed = true;
                    } else {
                        self.year[i] = num;
                        is_overflowed = false;
                        break;
                    }
                }
            }
            if is_overflowed {
                self.year.push(1);
            }
        }
    }
    pub fn add_millis(&mut self, millis: u64) {
        self.add_time(millis);
    }
    pub fn second(&self) -> u64 {
        (self.time % (60 * 1000)) / 1000
    }
    pub fn add_second(&mut self, second: u64) {
        self.add_millis(second * 1000);
    }
    pub fn minute(&self) -> u64 {
        (self.time % (60 * 60 * 1000)) / (60 * 1000)
    }
    pub fn add_minute(&mut self, minute: u64) {
        self.add_second(minute * 60);
    }
    pub fn hour(&self) -> u64 {
        (self.time % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000)
    }
    pub fn add_hour(&mut self, hour: u64) {
        self.add_minute(hour * 60);
    }
    pub fn day(&self) -> u64 {
        (self.time % (30 * 24 * 60 * 60 * 1000)) / (24 * 60 * 60 * 1000)
    }
    pub fn add_day(&mut self, day: u64) {
        self.add_hour(day * 24);
    }
    pub fn month(&self) -> u64 {
        (self.time % (12 * 30 * 24 * 60 * 60 * 1000)) / (30 * 24 * 60 * 60 * 1000)
    }
    pub fn add_month(&mut self, month: u64) {
        self.add_day(month * 30);
    }
    pub fn increase_year(&mut self) {
        self.add_years(vec![1]);
    }
}
pub fn year_to_str(cal: &GameCalender) -> String{
    match cal.year.iter().map(|&num| num.to_string()).rev().reduce(|s1, s2| s1.add(&s2)) {
        Some(result) => {
            result
        },
        None => {
            String::from("0")
        }
    }
}
fn u64_to_u8_arr(num: u64) -> Vec<u8> {
    let mut n = num;
    let mut result: Vec<u8> = Vec::new();
    while n >= 10 {
        let d = n / 10;
        let m = (n % 10) as u8;
        result.push(m);
        n = d;
    }
    result.push(n as u8);
    result
}
#[cfg(test)]
mod test {
    use crate::GameCalender;
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
    fn zero_year_to_string() {
        let cal = GameCalender::new();
        let year = year_to_str(&cal);

        assert_eq!(year, "0");
    }
    #[test]
    fn add_one_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(vec![1]);
        let year = year_to_str(&cal);

        assert_eq!(year, "1");
    }
    #[test]
    fn add_two_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(vec![1]);
        cal.add_years(vec![1]);
        let year = year_to_str(&cal);

        assert_eq!(year, "2");
    }
    #[test]
    fn add_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(vec![1]);
        cal.add_years(vec![1]);
        cal.add_month(7);
        cal.add_month(5);
        let year = year_to_str(&cal);
        let month = cal.month();

        assert_eq!(year, "3");
        assert_eq!(month, 0);
    }
}