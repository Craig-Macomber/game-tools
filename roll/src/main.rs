mod view;

#[derive(Debug, Default, Clone)]
struct State {
    lines: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Log {
    sync: bool,
    log: Vec<LogItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LogItem {
    markdown: String,
    // TODO: serializing local time is not ideal: should convert to utc
    timestamp: chrono::DateTime<Local>,
}

impl LogItem {
    fn new(markdown: String) -> Self {
        let timestamp = chrono::Local::now();
        LogItem {
            markdown,
            timestamp,
        }
    }
}

#[cfg(target_arch = "wasm32")]
use std::{collections::HashMap, sync::LazyLock};

use chrono::Local;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon_io/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(State {
            lines: load_default(),
        })
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        view::Body {}
    }
}

#[cfg(target_arch = "wasm32")]
fn load_url() -> Option<String> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let hash = location.hash().unwrap();
    if hash.starts_with("#") {
        let mut chars = hash.chars();
        chars.next();
        let str = chars.as_str();
        let decoded = js_sys::decode_uri_component(str)
            .unwrap()
            .as_string()
            .unwrap();
        Some(decoded)
    } else {
        None
    }
}

fn load_default() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return DEFAULT_TEXT.to_owned();
    }

    #[cfg(target_arch = "wasm32")]
    {
        return load_url().unwrap_or_else(|| {
            load_storage(STORAGE_KEY).unwrap_or_else(|| DEFAULT_TEXT.to_owned())
        });
    }
}

static DEFAULT_TEXT: &'static str = r#"# Examples:
1d20
2d6 + 1d4 + 5

# Advantage:
2d20 K1

# Disadvantage:
2d20 k1

# Repeated rolls:
(2d6 + 6) ^+ 8

# Re-roll ones:
8d6 r1

# Inline
**Stab:** <Roll d="1d20+5"/> to hit dealing <Roll d="2d6"/> on hit.

**Other:** <Roll d="1d20+5"/> to hit dealing <Roll d="2d6"/> **cold** damage on hit.

# Custom Tags

## Attack
<A m="5" d="2d6 + 1d8" f="2"/>

## Counter:
Health: <Counter initial="20"/>
"#;

#[cfg(target_arch = "wasm32")]
fn save_url(data: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let encoded = js_sys::encode_uri_component(data).as_string().unwrap();
    location.set_hash(&encoded).unwrap();
}

#[cfg(target_arch = "wasm32")]
static STORAGE_KEY: &'static str = "roller: text";

#[cfg(target_arch = "wasm32")]
fn save_storage(key: &str, data: Option<&str>) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    match data {
        Some(data) => storage.set_item(key, data).unwrap(),
        None => storage.remove_item(key).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
fn load_storage(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().unwrap_or(None)?;
    storage.get_item(key).unwrap_or(None)
}

#[cfg(target_arch = "wasm32")]
struct OnStorage {
    callbacks: HashMap<String, Vec<Box<dyn Fn() -> CallbackRetention + Sync + Send>>>,
}

pub enum CallbackRetention {
    Keep,
    Remove,
}

#[cfg(target_arch = "wasm32")]
static ON_STORAGE: LazyLock<std::sync::Mutex<OnStorage>> = LazyLock::new(|| {
    use wasm_bindgen::prelude::*;

    let window = web_sys::window().unwrap();
    assert!(window.onstorage().is_none());

    let closure: Closure<dyn FnMut(web_sys::StorageEvent)> =
        Closure::new(move |event: web_sys::StorageEvent| {
            // https://developer.mozilla.org/en-US/docs/Web/API/Window/storage_event#event_properties
            let s = &mut *ON_STORAGE.lock().unwrap();
            let key = event.key().unwrap();
            s.callbacks.entry(key).and_modify(|callbacks| {
                callbacks.retain(|callback| match callback() {
                    CallbackRetention::Keep => true,
                    CallbackRetention::Remove => false,
                });
            });
        });

    window.set_onstorage(Some(wasm_bindgen::JsCast::unchecked_ref(closure.as_ref())));
    closure.forget(); // Leak closure: since this is part of a static it living forever is fine.

    OnStorage {
        callbacks: HashMap::new(),
    }
    .into()
});

#[cfg(target_arch = "wasm32")]
pub fn on_storage(key: &str, callback: Box<dyn Fn() -> CallbackRetention + Sync + Send>) {
    let s = &mut *ON_STORAGE.lock().unwrap();
    s.callbacks
        .entry(key.to_string())
        .or_default()
        .push(callback);
}

#[cfg(not(target_arch = "wasm32"))]
fn save_url(_data: &str) {}
