use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MIN_DAY: i32 = 24 * 60;
const MIN_HOUR: i32 = 60;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MIN_HOUR,
            self.minutes % MIN_HOUR
        )
    }
}

impl Clock {
    pub fn new(_hours: i32, _minutes: i32) -> Self {
        Clock {
            minutes: (((_hours * MIN_HOUR + _minutes) % MIN_DAY) + MIN_DAY) % MIN_DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}
