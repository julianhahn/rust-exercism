use core::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        if self.minutes + minutes < 60 {
            self.minutes = self.minutes + minutes;
        } else {
            let remaining = (self.minutes + minutes) - 60;
            self.hours += 1;
            self.minutes = remaining
        }
    }
}
