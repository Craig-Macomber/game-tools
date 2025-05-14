use chrono::{DateTime, Local};
use dioxus::prelude::*;

use std::{future::Future, sync::Arc};

use super::time_observer::TimeObserver;

/// Workaround from https://github.com/DioxusLabs/dioxus/issues/4114
fn schedule_update_fixed() -> Arc<dyn Fn() + Send + Sync + 'static> {
    let subscribers: Arc<std::sync::Mutex<std::collections::HashSet<ReactiveContext>>> =
        Default::default();

    if let Some(reactive_context) = ReactiveContext::current() {
        reactive_context.subscribe(subscribers.clone());
    }

    let callback = move || {
        for reactive_context in subscribers.lock().unwrap().iter() {
            reactive_context.mark_dirty();
        }
    };
    Arc::new(callback)
}

/// Provide a callback access to the current time,
/// and schedule an dioxus update for when what was observed about the time will change.
pub fn observe_time<T, F: FnOnce(&mut TimeObserver) -> T>(f: F) -> T {
    let now = Local::now();
    let mut observer = TimeObserver::new(now);
    let result = f(&mut observer);

    if let Some(deadline) = observer.into_deadline() {
        let update = schedule_update_fixed();

        #[cfg(target_arch = "wasm32")]
        if (deadline - now) <= chrono::TimeDelta::milliseconds(50) {
            dioxus::logger::tracing::trace!("do animation");
            use wasm_bindgen::{closure, JsCast};
            let closure = closure::Closure::once_into_js(move || update());
            let window = web_sys::window().expect("no global `window` exists");
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .unwrap();
            return result;
        }

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
    // TODO: tests
}
