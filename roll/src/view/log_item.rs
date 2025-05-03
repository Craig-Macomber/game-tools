use chrono::{DateTime, Local};
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::LogItem;

use std::{future::Future, vec};

use super::time_observer::TimeObserver;

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

/// Provide a callback access to the current time,
/// and schedule an dioxus update for when what was observed about the time will change.
fn observe_time<T, F: FnOnce(&mut TimeObserver) -> T>(f: F) -> T {
    let mut observer = TimeObserver::new(Local::now());
    let result = f(&mut observer);
    if let Some(deadline) = observer.into_deadline() {
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
    let seconds = f32::max(0f32, duration.as_seconds_f32());
    async_std::task::sleep(std::time::Duration::from_secs_f32(seconds))
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::*;

    #[test]
    fn display_time() {
        let t = Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
        let now = Local.with_ymd_and_hms(2014, 7, 8, 9, 15, 11).unwrap();
        let s = display_relative_time_inner(t, &mut TimeObserver::new(now));
        assert_eq!(s, "09:10:11 (5 minutes ago)");
    }
}
