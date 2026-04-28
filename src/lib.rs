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
    pub fn add_tick(&mut self, amount: u8) {
        let mut result: u8;
        let mut is_overflowed: bool;
        (result, is_overflowed) = self.tick[0].overflowing_add(amount);
        self.tick[0] = result;
        for i in 1..self.tick.len() {
            if is_overflowed {
                (result, is_overflowed) = self.tick[i].overflowing_add(1);
                self.tick[i] = result;
            } else {
                break;
            }
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
        let bytes = millis.to_le_bytes();
        let mut result: u8;
        let mut is_overflowed: bool;
        let mut prev_is_overflowed: bool = false;
        for i in 0..bytes.len() {
            (result, is_overflowed) = self.tick[i].overflowing_add(bytes[i]);
            if is_overflowed {
                if prev_is_overflowed {
                    result = result + 1;
                }
                prev_is_overflowed = true;
            } else {
                if prev_is_overflowed {
                    (result, prev_is_overflowed) = result.overflowing_add(1);
                }
            }
            self.tick[i] = result;
        }
        is_overflowed = prev_is_overflowed;
        for i in 4..self.tick.len() {
            if is_overflowed {
                (result, is_overflowed) = self.tick[i].overflowing_add(1);
                self.tick[i] = result;
            } else {
                break;
            }
        }
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
}
#[cfg(test)]
mod test {
    use crate::GameCalender;

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
}