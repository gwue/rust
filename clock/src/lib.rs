const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

#[derive(Debug)]
pub struct Clock {
    raw_minutes: i32,
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            raw_minutes: modulo(hours * MINUTES_PER_HOUR + minutes, MINUTES_PER_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            raw_minutes: modulo(self.raw_minutes + minutes, MINUTES_PER_DAY),
        }
    }

    fn get_minutes(&self) -> i32 {
        self.raw_minutes % 60
    }

    fn get_hours(&self) -> i32 {
        self.raw_minutes / 60
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.raw_minutes == other.raw_minutes
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.get_hours(), self.get_minutes())
    }
}

impl std::ops::Add for &Clock {
    type Output = Clock;

    fn add(self, other: Self) -> Self::Output {
        self.add_minutes(other.raw_minutes)
    }
}

impl std::ops::Add<i32> for &Clock {
    type Output = Clock;

    fn add(self, other: i32) -> Self::Output {
        self.add_minutes(other)
    }
}
