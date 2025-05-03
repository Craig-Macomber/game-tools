use chrono::{DateTime, Local, TimeDelta};
use dioxus::{logger::tracing, prelude::*};
use dioxus_markdown::Markdown;

use crate::LogItem;

use std::{future::Future, vec};

/**
 * Display log item.
 */
#[component]
pub(crate) fn LogItemView(item: LogItem) -> Element {
    rsx!(
        // TODO: proper accessible tooltip
        // TODO: some animation to highlight new items (fade in? Use monotonic time?)
        span { title: "{display_relative_time(item.timestamp)}",
            Markdown { src: "{item.markdown}" }
        }
    )
}

fn display_relative_time(time: chrono::DateTime<Local>) -> String {
    observe_time(|t| display_relative_time_inner(time, t))
}

fn display_relative_time_inner(time: chrono::DateTime<Local>, now: &mut TimeObserver) -> String {
    if now.in_future(time) {
        // Include full date
        return time.format("%c").to_string();
    }

    let hours_ago = -now.hours_after_now(time);

    let time_string = if hours_ago > 8 {
        // Include full date
        time.format("%c").to_string()
    } else {
        // Just show time
        time.format("%X").to_string()
    };

    // TODO: would be nice to use some concise relative time utility here with localization support.
    let ago_string = if hours_ago > 1 {
        format!("{hours_ago} hours ago")
    } else {
        let minutes_ago = -now.minutes_after_now(time);
        if minutes_ago > 1 {
            format!("{minutes_ago} minutes ago")
        } else {
            let seconds_ago = -now.seconds_after_now(time);
            let s = if seconds_ago == 1 { "" } else { "s" };
            format!("{seconds_ago} second{s} ago")
        }
    };

    format!("{time_string} ({ago_string})")
}

pub struct TimeObserver {
    now: DateTime<Local>,
    /// When something observed about `now` would change (relative to now)
    wake: Option<TimeDelta>,
}

impl TimeObserver {
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

    fn wake(&mut self, d: TimeDelta) {
        if let Some(old) = self.wake {
            if old < d {
                return;
            }
        }
        self.wake = Some(d);
    }
}

fn observe_time<T, F: FnOnce(&mut TimeObserver) -> T>(f: F) -> T {
    let mut observer = TimeObserver {
        now: Local::now(),
        wake: None,
    };
    let result = f(&mut observer);
    if let Some(wake) = observer.wake {
        let deadline = observer.now + wake;
        let update = schedule_update();
        spawn(async move {
            sleep_until(deadline).await;
            update();
        });
    }
    result
}

fn sleep_until(deadline: DateTime<Local>) -> impl Future<Output = ()> {
    // Ideally would use tokio::time::sleep_until(deadline), but that uses both a different clock and different types.
    // instead hack something mostly correct:
    // This does not account for if the local clock is changed since desired delay is in local time not monotonic time, but sleep is in monotonic time.
    // TODO: this could be made better by detecting clock changes somehow (or just waking periodically) and restarting the sleep afterwards with newly computed delta.
    let duration = deadline - Local::now();
    let seconds = duration.as_seconds_f32();
    async_std::task::sleep(std::time::Duration::from_secs_f32(f32::max(0f32, seconds)))
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeDelta, TimeZone};

    use super::*;

    #[test]
    fn display_time() {
        let t = Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
        let now = Local.with_ymd_and_hms(2014, 7, 8, 9, 15, 11).unwrap();
        let s = display_relative_time_inner(t, &mut TimeObserver { now, wake: None });
        assert_eq!(s, "09:10:11 (5 minutes ago)");
    }

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
