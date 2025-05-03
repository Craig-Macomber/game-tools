use chrono::DateTime;
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
    pub fn new(now: DateTime<Local>) -> Self {
        TimeObserver { now, wake: None }
    }

    pub fn in_future(&mut self, t: DateTime<Local>) -> bool {
        let delta = t - self.now;
        if delta > TimeDelta::zero() {
            self.wake(delta);
            true
        } else {
            false
        }
    }

    pub fn hours_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let hours = delta.num_hours();
        let wake = delta - TimeDelta::hours(if hours > 0 { hours } else { hours - 1 });
        self.wake(wake);
        hours
    }

    pub fn minutes_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let minutes = delta.num_minutes();
        let wake = delta - TimeDelta::minutes(if minutes > 0 { minutes } else { minutes - 1 });
        self.wake(wake);
        minutes
    }

    pub fn seconds_after_now(&mut self, t: DateTime<Local>) -> i64 {
        let delta = t - self.now;
        let seconds = delta.num_seconds();
        let wake = delta - TimeDelta::seconds(if seconds > 0 { seconds } else { seconds - 1 });
        self.wake(wake);
        seconds
    }

    pub(crate) fn wake(&mut self, d: TimeDelta) {
        if let Some(old) = self.wake {
            if old < d {
                return;
            }
        }
        self.wake = Some(d);
    }

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
