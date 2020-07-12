#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl<'a> Clock {
    const HOURS_IN_DAY: i32 = 24;
    const MINUTES_IN_HOUR: i32 = 60;
    const TIME_SEPARATOR: &'a str = ":";

    pub fn new(hours: i32, minutes: i32) -> Self {
        let time_from_minutes = Clock::compute_time_from_minutes(minutes);

        Clock {
            hours: Clock::compute_hours(hours + time_from_minutes.0),
            minutes: time_from_minutes.1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn compute_time_from_minutes(minutes: i32) -> (i32, i32) {
        let clamped_minutes = minutes % Clock::MINUTES_IN_HOUR;
        let mut additional_hours = (minutes - clamped_minutes) / Clock::MINUTES_IN_HOUR;

        let minutes_on_clock = if clamped_minutes < 0 {
            additional_hours -= 1;
            clamped_minutes + Clock::MINUTES_IN_HOUR
        } else {
            clamped_minutes
        };

        (additional_hours, minutes_on_clock)
    }

    fn compute_hours(hours: i32) -> i32 {
        let clamped_hours = hours % Clock::HOURS_IN_DAY;
        let hours_on_clock = if clamped_hours < 0 {
            clamped_hours + Clock::HOURS_IN_DAY
        } else {
            clamped_hours
        };

        hours_on_clock
    }

    fn format(time: i32) -> String {
        let formatted_time = if time < 10 {
            format!("0{}", time)
        } else {
            time.to_string()
        };

        formatted_time
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            Clock::format(self.hours),
            Clock::TIME_SEPARATOR,
            Clock::format(self.minutes)
        )
    }
}
