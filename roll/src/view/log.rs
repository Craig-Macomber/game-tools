use dioxus::{logger::tracing::trace, prelude::*};
use serde::{Deserialize, Serialize};

mod log_item;

use std::{borrow::Borrow, vec};

#[cfg(target_arch = "wasm32")]
use crate::{load_storage, on_storage, save_storage, CallbackRetention};

use crate::{Log, LogItem};

/**
 * Display Log.
 */
#[component]
pub(crate) fn LogView() -> Element {
    // The custom storage logic is similar to https://docs.rs/dioxus-sdk/0.6.0/dioxus_sdk/storage/fn.use_synced_storage.html
    // but with a few differences:
    // 1. Uses a human readable format (pretty printed JSON instead of compressed postcard)
    // 2. Supports fully removing/cleating the storage.
    // 3. Only supports wasm/browser.
    //
    // TODO:
    // Integrate with with Dioxus's storage system: https://github.com/DioxusLabs/sdk/pull/85
    rsx!(
        h2 { "Log:" }
        LogStorage {}
        for message in LOG.read().borrow().log.iter().rev() {
            log_item::LogItemView { item: message.clone() }
        }
    )
}

#[cfg(target_arch = "wasm32")]
static LOG_STORAGE_KEY: &'static str = "roller: log";

#[cfg(target_arch = "wasm32")]
#[component]
pub fn LogStorage() -> Element {
    let sync = LOG.read().sync;

    if sync {
        // Side-effect: update local storage
        // Since this component observes the log, it will rerender when ever it changes, and thus can keep the sync up to date.
        write_log_to_local_storage(&LOG.read().log);
    }

    rsx!(
        span {
            ClearButton {}
            LoadButton {}
            input {
                r#type: "checkbox",
                id: "Save",
                oninput: move |_| {
                    if !sync {
                        write_log_to_local_storage(&LOG.read().log)
                    }
                    LOG.write().sync = !sync;
                },
                checked: sync,
            }
            label { r#for: "Save", "Sync with Local Storage" }
        }
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn LogStorage() -> Element {
    rsx!()
}

#[cfg(target_arch = "wasm32")]
fn write_log_to_local_storage(log: &Vec<LogItem>) {
    // if load_storage(LOG_STORAGE_KEY).is_none() && log.is_empty() {
    //     // Lets clear with sync actually work, and return to the default state of not using local storage for the log.
    //     // This also means someone using sync mode will remove default sync mode when clearing if refreshing before logging anything,
    //     // which doesn't seem like a consistent behavior.
    //     return;
    // }

    trace!("writing log");
    let to_encode = EncodedLog { log: log.clone() };
    let encoded = serde_json::to_string_pretty(&to_encode).unwrap();
    save_storage(LOG_STORAGE_KEY, Some(&encoded));
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn Clear() -> Element {
    rsx!(ClearButton {})
}

#[component]
pub fn ClearButton() -> Element {
    let sync = LOG.read().sync;
    rsx!(
        button {
            onclick: move |_| {
                *LOG.write() = Log { sync, log: Vec::default() };
            },
            "Clear"
        }
    )
}

#[component]
pub fn LoadButton() -> Element {
    let sync = LOG.read().sync;
    if sync {
        rsx!(
            button {
                onclick: move |_| {
                    LOG.write().sync = false;
                    #[cfg(target_arch = "wasm32")]
                    if sync {
                        save_storage(LOG_STORAGE_KEY, None)
                    }
                },
                "Disable Local Storage"
            }
        )
    } else {
        rsx!(
            button {
                onclick: move |_| {
                    *LOG.write() = load_or_new_log(false);
                    LOG.write().sync = true;
                },
                "Load Local Storage"
            }
        )
    }
}

pub(crate) fn load_or_new_log(skip_load_message: bool) -> Log {
    #[cfg(target_arch = "wasm32")]
    if let Some(stored) = load_storage(LOG_STORAGE_KEY) {
        trace!("Loading existing log from local storage!");
        let mut log = vec![];

        let parsed: Result<EncodedLog, serde_json::Error> = serde_json::from_str(&stored);
        let mut sync = false;

        match parsed {
            Ok(loaded) => {
                log.extend(loaded.log);
                if !skip_load_message {
                    log.push(LogItem::new("*Loaded from Storage*".to_string()));
                }
                sync = true;
            }
            Err(e) => log.push(LogItem::new(format!("Failed to parse stored log: {e}"))),
        }

        return Log { sync, log };
    }

    trace!("Loading default log");

    Log {
        sync: false,
        log: Vec::default(),
    }
}

#[derive(Serialize, Deserialize)]
struct EncodedLog {
    log: Vec<LogItem>,
}

pub static LOG: GlobalSignal<Log> = Signal::global(|| {
    #[cfg(target_arch = "wasm32")]
    on_storage(
        LOG_STORAGE_KEY,
        Box::new(move || {
            if LOG.read().sync {
                let new = load_or_new_log(true);
                if !new.eq(&*LOG.read()) {
                    *LOG.write() = new;
                }
            }
            CallbackRetention::Keep
        }),
    );
    load_or_new_log(false)
});
