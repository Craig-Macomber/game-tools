use chrono::{DateTime, Duration, Local};
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

/// Provide a callback access to the current time,
/// and schedule an dioxus update for when what was observed about the time will change.
fn use_time<T, F: FnOnce(&mut TimeObserver) -> T>(f: F) -> T {
    let now = Local::now();
    let mut signal = use_signal(|| now);
    // Suppress future updates for deadlines that have already passed.
    *signal.write_silent() = now;

    // Indicate a dependency on this signal so writing to it can be used to trigger an update.
    signal.read();

    let mut observer = TimeObserver::new(now);
    let result = f(&mut observer);

    if let Some(deadline) = observer.into_deadline() {
        // TODO: It seems like this should work:
        // let update = schedule_update();
        // However using that doesn't cancel the timeout after the context is rerendered, causing multiple timeouts to accumulate, each triggering more timeouts.
        // There should be some way to fix this that doesn't require making a signal to deduplicate timeouts.
        // If the need for a signal is removed, then `use_time` could stop being a hook and renamed to something like `observe_time`.

        let mut update = move || {
            let now = Local::now();
            if *signal.peek() < deadline {
                dioxus::logger::tracing::trace!("wake");
                signal.set(now);
            }
        };

        #[cfg(target_arch = "wasm32")]
        if (deadline - now) <= chrono::TimeDelta::milliseconds(50) {
            dioxus::logger::tracing::trace!("do animation");
            use wasm_bindgen::{closure, JsCast};
            let closure = closure::Closure::once_into_js(update);
            let window = web_sys::window().expect("no global `window` exists");
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .unwrap();
            return result;
        }

        // TODO: For platforms other than wasm32, implement some animation/update throttling.

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
    dioxus::logger::tracing::trace!("Sleeping for {seconds}");
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
        let s = format_relative_time(t, &mut TimeObserver::new(now));
        assert_eq!(s, "09:10:11 (5 minutes ago)");
    }
}
