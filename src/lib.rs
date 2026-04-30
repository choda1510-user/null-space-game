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
    pub fn from_time(time: u64) -> GameCalender {
        GameCalender {
            year: Vec::from([0 as u8]), 
            time: time
        }
    }
    pub fn from_years(years: Vec<u8>) -> GameCalender {
        GameCalender {
            year: years,
            time: 0
        }
    }
    pub fn from_date(years: Vec<u8>, time: u64) -> GameCalender {
        GameCalender {
            year: years,
            time: time
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
            self.add_years(&u64_to_u8_arr(d3));
            self.time = m3;
        } else {
            self.add_years(&u64_to_u8_arr(d1));
            self.time = m1;
        }
    }

    pub fn add_years(&mut self, years: &Vec<u8>) {
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
    pub fn add_calender(&mut self, cal: &GameCalender) {
        self.add_years(&cal.year);
        self.add_time(cal.time);
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
    pub fn year(&self) -> &Vec<u8> {
        &self.year
    }
    pub fn increase_year(&mut self) {
        self.add_years(&vec![1]);
    }
}
impl PartialEq for GameCalender {
    fn eq(&self, other: &Self) -> bool {
        if self.year.eq(&other.year) && self.time == other.time {
            true
        } else {
            false
        }
    }
}
impl PartialOrd for GameCalender {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for GameCalender {

}
impl Ord for GameCalender {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.year.eq(&other.year) {
            if self.time == other.time {
                std::cmp::Ordering::Equal
            } else {
                if self.time > other.time {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
        } else {
            if self.year.year_gt(&other.year) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
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
impl Clone for GameCalender {
    fn clone(&self) -> Self {
        GameCalender {
            year: self.year.clone(),
            time: self.time,
        }
    }
}
pub fn str_to_year(src: &str) -> Option<Vec<u8>> {
    if src.chars().count() == 0 {
        return None;
    }
    if src.chars().any(|c| !c.is_digit(10) && c != ',') {
        return None;
    }
    let result = src.chars()
        .filter(|&c| c.is_digit(10))
        .rev()
        .filter_map(|c| c.to_string().parse::<u8>().ok())
        .collect::<Vec<u8>>();
    if result.len() == 0 {
        return None;
    } else {
        return Some(result);
    }
}
pub fn e_str_to_year(src: &str) -> Option<Vec<u8>> {
    if src.chars().count() == 0 {
        return None;
    }
    enum State {
        First,
        Dot,
        ZeroDot,
        Significand,
        ZeroSignificand,
        Exponent,
        ZeroExponent,
    }
    let mut result = Vec::new();
    let mut exponent = String::new();
    let mut state = State::First;
    for ch in src.chars() {
        match state {
            State::First => {
                match ch.to_string().parse::<u8>() {
                    Ok(n) => {
                        result.push(n);
                        state = if n != 0 {State::Dot} else {State::ZeroDot};
                    },
                    Err(_error) => {
                        return None;
                    }
                }
            },
            State::Dot => {
                if ch != '.' {
                    return None;
                } else {
                    state = State::Significand;
                }
            }
            State::ZeroDot => {
                if ch != '.' {
                    return None;
                } else {
                    state = State::ZeroSignificand;
                }
            }
            State::Significand => {
                if ch.is_digit(10) {
                    match ch.to_string().parse::<u8>() {
                        Ok(n) => {
                            result.insert(0, n);
                        },
                        Err(_error) => {
                            return None;
                        }
                    }
                } else if ch == 'e' || ch == 'E' {
                    state = State::Exponent;
                }
            },
            State::ZeroSignificand => {
                if ch.is_digit(10) {
                    match ch.to_string().parse::<u8>() {
                        Ok(n) => {
                            if n != 0 {
                                return None;
                            }
                        },
                        Err(_error) => {
                            return None;
                        }
                    }
                } else if ch == 'e' || ch == 'E' {
                    exponent = exponent.add("0");
                    state = State::ZeroExponent;
                }
            }
            State::Exponent => {
                exponent = exponent.add(&ch.to_string());
            },
            State::ZeroExponent => {

            }
        }
    }
    loop {
        match result.get(0) {
            Some(&n) => {
                if n == 0 {
                    result.remove(0);
                } else {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }
    match exponent.parse::<usize>() {
        Ok(ex) => {
            for _ in 0..ex {
                result.insert(0, 0);
            }
        },
        Err(_error) => {
            return None
        }
    }
    Some(result)
}
pub trait GameYear {
    fn year_lt(&self, other: &Self) -> bool;
    fn year_le(&self, other: &Self) -> bool;
    fn year_gt(&self, other: &Self) -> bool;
    fn year_ge(&self, other: &Self) -> bool;
}
impl GameYear for Vec<u8> {
    fn year_lt(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            return false;
        }
        let mut than = false;
        for i in 0..self.len() {
            if self[i] < other[i] {
                than = true;
            }
            if self[i] > other[i] {
                than = false;
            }
        }
        than
    }
    fn year_le(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            return false;
        }
        let mut than = true;
        for i in 0..self.len() {
            if self[i] < other[i] {
                than = true;
            }
            if self[i] > other[i] {
                than = false;
            }
        }
        than
    }
    fn year_gt(&self, other: &Self) -> bool {
        if other.len() > self.len() {
            return false;
        }
        let mut than = false;
        for i in 0..other.len() {
            if other[i] < self[i] {
                than = true;
            }
            if other[i] > self[i] {
                than = false;
            }
        }
        than
    }
    fn year_ge(&self, other: &Self) -> bool {
        if other.len() > self.len() {
            return false;
        }
        let mut than = true;
        for i in 0..other.len() {
            if other[i] < self[i] {
                than = true;
            }
            if other[i] > self[i] {
                than = false;
            }
        }
        than
    }
}
#[cfg(test)]
mod test {
    use crate::{GameCalender, e_str_to_year, str_to_year};

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
        let year = cal.year()[0].to_string();

        assert_eq!(year, "0");
    }
    #[test]
    fn add_one_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(&vec![1]);
        let year = cal.year()[0].to_string();

        assert_eq!(year, "1");
    }
    #[test]
    fn add_two_year_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(&vec![1]);
        cal.add_years(&vec![1]);
        let year = cal.year()[0].to_string();

        assert_eq!(year, "2");
    }
    #[test]
    fn add_tick_to_string() {
        let mut cal = GameCalender::new();
        cal.add_years(&vec![1]);
        cal.add_years(&vec![1]);
        cal.add_month(7);
        cal.add_month(5);
        let year = cal.year()[0].to_string();
        let month = cal.month();

        assert_eq!(year, "3");
        assert_eq!(month, 0);
    }
    #[test]
    fn str_to_year_test() {
        let years0 = str_to_year("0");
        assert!(years0.is_some());
        let years1 = str_to_year("2000");
        assert!(years1.is_some());
        let cal1 = GameCalender::from_years(years1.unwrap());
        let years2 = str_to_year("2,000");
        assert!(years2.is_some());
        let cal2 = GameCalender::from_years(years2.unwrap());
        assert_eq!(cal1.year, cal2.year);
        let invalid1 = str_to_year("");
        assert!(invalid1.is_none());
        let invalid2 = str_to_year("abcd");
        assert!(invalid2.is_none());
        let invalid3 = str_to_year("15.000");
        assert!(invalid3.is_none());
    }
    #[test]
    fn e_str_to_year_test() {
        let year_zero1 = e_str_to_year("0.e0");
        assert!(year_zero1.is_some());
        let cal_zero1 = GameCalender::from_years(year_zero1.unwrap());
        let year_zero2 = e_str_to_year("0.0e0");
        assert!(year_zero2.is_some());
        let cal_zero2 = GameCalender::from_years(year_zero2.unwrap());
        assert_eq!(cal_zero1.year, cal_zero2.year);
        let year1 = e_str_to_year("4.1e13");
        assert!(year1.is_some());
        let year2 = e_str_to_year("5.e0");
        assert!(year2.is_some());
        let cal2 = GameCalender::from_years(year2.unwrap());
        let year3 = e_str_to_year("5.0e0");
        assert!(year3.is_some());
        let cal3 = GameCalender::from_years(year3.unwrap());
        assert_eq!(cal2.year, cal3.year);
    }
    #[test]
    fn str_and_e_str_to_year_test() {
        let year1 = str_to_year("1,000,000,000,000");
        let cal1 = GameCalender::from_years(year1.unwrap());
        let year2 = e_str_to_year("1.e12");
        let cal2 = GameCalender::from_years(year2.unwrap());
        assert_eq!(cal1.year, cal2.year);
    }
}