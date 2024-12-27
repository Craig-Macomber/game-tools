use serde::{Deserialize, Serialize};

mod view;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct State {
    lines: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
            lines: load_default().unwrap_or("".to_owned()),
        })
    });
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        view::Body {}
    }
}

#[cfg(target_arch = "wasm32")]
fn load_default() -> Option<String> {
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

#[cfg(not(target_arch = "wasm32"))]
fn load_default() -> Option<String> {}

#[cfg(target_arch = "wasm32")]
fn save_default(data: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let encoded = js_sys::encode_uri_component(data).as_string().unwrap();
    location.set_hash(&encoded).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn save_default(data: &str) {}
