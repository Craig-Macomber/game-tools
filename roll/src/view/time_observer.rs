use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::TimeDelta;

pub struct TimeObserver {
    now: DateTime<Local>,
    /// When something observed about `now` would change (relative to now).
    ///
    /// TODO: consider a 3 state enum here allowing for an "animating" state that observed the exact time.
    /// Use this for cases like animations what end after some amount of time, or are periodic.
    wake: Option<TimeDelta>,
}

impl TimeObserver {
    /// Create a new TimeObserver which treats the current time as `now`.
    pub fn new(now: DateTime<Local>) -> Self {
        TimeObserver { now, wake: None }
    }

    /// True if `t` is in the future.
    pub fn in_future(&mut self, t: DateTime<Local>) -> bool {
        let delta = t - self.now;
        if delta > TimeDelta::zero() {
            self.wake(delta);
            true
        } else {
            false
        }
    }

    /// How far in the future is `t` in days.
    ///
    /// Rounded toward zero.
    pub fn days_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let days = delta.num_days();
        let wake = delta - TimeDelta::days(if days > 0 { days } else { days - 1 });
        self.wake(wake);
        days
    }

    /// How far in the future is `t` in hours.
    ///
    /// Rounded toward zero.
    pub fn hours_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let hours = delta.num_hours();
        let wake = delta - TimeDelta::hours(if hours > 0 { hours } else { hours - 1 });
        self.wake(wake);
        hours
    }

    /// How far in the future is `t` in minutes.
    ///
    /// Rounded toward zero.
    pub fn minutes_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let minutes = delta.num_minutes();
        let wake = delta - TimeDelta::minutes(if minutes > 0 { minutes } else { minutes - 1 });
        self.wake(wake);
        minutes
    }

    /// How far in the future is `t` in seconds.
    ///
    /// Rounded toward zero.
    pub fn seconds_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let seconds = delta.num_seconds();
        let wake = delta - TimeDelta::seconds(if seconds > 0 { seconds } else { seconds - 1 });
        self.wake(wake);
        seconds
    }

    /// Returns a value [0, 1] based on how far from start to end the current time is.
    ///
    /// * `precision` - how far off the value can get before waking up to get a new one. Provide 0.0 to animate as fast as possible.
    pub fn lerp(&mut self, start: DateTime<Local>, end: DateTime<Local>, precision: f32) -> f32 {
        if !self.in_future(end) {
            return 1.0;
        }

        if self.in_future(start) {
            return 0.0;
        }

        let current = self.now - start;
        let length = end - start;
        let phase = current.as_seconds_f32() / length.as_seconds_f32();
        debug_assert!(phase >= 0.0);
        debug_assert!(phase <= 1.0);

        self.wake(
            Duration::from_std(std::time::Duration::from_secs_f32(
                length.as_seconds_f32() * precision,
            ))
            .unwrap(),
        );

        phase
    }

    fn wake(&mut self, d: TimeDelta) {
        if let Some(old) = self.wake {
            if old < d {
                return;
            }
        }
        self.wake = Some(d);
    }

    /// Get the deadline at which the results of queries made on this TimeObserver will become out of date.
    pub fn into_deadline(self) -> Option<DateTime<Local>> {
        self.wake.map(|delta| self.now + delta)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeDelta, TimeZone};

    use super::*;

    #[test]
    fn observe_time() {
        let mut observer = TimeObserver {
            now: Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(),
            wake: None,
        };

        assert_eq!(
            observer.in_future(Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 10).unwrap()),
            false
        );

        assert!(observer.wake.is_none());

        assert_eq!(
            observer.in_future(Local.with_ymd_and_hms(2014, 7, 9, 9, 10, 11).unwrap()),
            true
        );

        assert_eq!(observer.wake, Some(TimeDelta::days(1)));

        let h = observer.hours_after_now(Local.with_ymd_and_hms(2014, 7, 8, 9, 12, 11).unwrap());
        assert_eq!(h, 0);
        assert_eq!(observer.wake, Some(TimeDelta::minutes(62)));

        let h = observer.hours_after_now(Local.with_ymd_and_hms(2014, 7, 8, 9, 8, 11).unwrap());
        assert_eq!(h, 0);
        assert_eq!(observer.wake, Some(TimeDelta::minutes(58)));
    }
}
