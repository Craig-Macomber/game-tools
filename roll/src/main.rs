mod view;

#[derive(Debug, Default, Clone)]
struct State {
    lines: String,
}

#[derive(Debug, Default, Clone)]
struct Log {
    log: Vec<String>,
}

use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // Instead of using the assert, inline the css to work around absolute path issues in deployment.
        // style { {include_str!("../assets/main.css")} }

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
        return load_url()
            .unwrap_or_else(|| load_storage().unwrap_or_else(|| DEFAULT_TEXT.to_owned()));
    }
}

static DEFAULT_TEXT: &'static str = "## Examples:
1d20
2d6 + 1d4 + 5

## Advantage:
2d20 K1

## Disadvantage:
2d20 k1

## Repeated rolls:
(2d6 + 6) ^+ 8

## Re-roll ones:
8d6 r1";

#[cfg(target_arch = "wasm32")]
fn save_url(data: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let encoded = js_sys::encode_uri_component(data).as_string().unwrap();
    location.set_hash(&encoded).unwrap();
}

static STORAGE_KEY: &'static str = "roller: text";

#[cfg(target_arch = "wasm32")]
fn save_storage(data: Option<&str>) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    match data {
        Some(data) => storage.set_item(STORAGE_KEY, data).unwrap(),
        None => storage.remove_item(STORAGE_KEY).unwrap(),
    }
}

#[cfg(target_arch = "wasm32")]
fn load_storage() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().unwrap_or(None)?;
    storage.get_item(STORAGE_KEY).unwrap_or(None)
}

#[cfg(not(target_arch = "wasm32"))]
fn save_url(_data: &str) {}
