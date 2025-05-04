use chrono::{Duration, Local};
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::LogItem;

use std::vec;

use super::{dioxus_time::use_time, time_observer::TimeObserver};

/**
 * Display log item.
 */
#[component]
pub(crate) fn LogItemView(item: LogItem) -> Element {
    use_time(|t| {
        // TODO: this should probably use CSS animation for better efficiency.
        let fade = t.lerp(item.timestamp, item.timestamp + Duration::seconds(1), 0.002);
        rsx!(
            // TODO: proper accessible tooltip
            span {
                title: "{format_relative_time(item.timestamp, t)}",
                opacity: fade,
                Markdown { src: "{item.markdown}" }
            }
        )
    })
}

fn format_relative_time(to_format: chrono::DateTime<Local>, now: &mut TimeObserver) -> String {
    if now.in_future(to_format) {
        // Include full date
        return to_format.format("%c").to_string();
    }

    let hours_ago = -now.hours_after_now(to_format);

    let time_string = if hours_ago > 8 {
        // Include full date
        to_format.format("%c").to_string()
    } else {
        // Just show time
        to_format.format("%X").to_string()
    };

    // TODO: would be nice to use some concise relative time utility here with localization support.
    let ago_string = if hours_ago > 1 {
        format!("{hours_ago} hours ago")
    } else {
        let minutes_ago = -now.minutes_after_now(to_format);
        if minutes_ago > 1 {
            format!("{minutes_ago} minutes ago")
        } else {
            let seconds_ago = -now.seconds_after_now(to_format);
            let s = if seconds_ago == 1 { "" } else { "s" };
            format!("{seconds_ago} second{s} ago")
        }
    };

    format!("{time_string} ({ago_string})")
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::*;

    #[test]
    fn display_time() {
        let t = Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();
        let now = Local.with_ymd_and_hms(2014, 7, 8, 9, 15, 11).unwrap();
        let s = format_relative_time(t, &mut TimeObserver::new(now));
        assert_eq!(s, "09:10:11 (5 minutes ago)");
    }
}
