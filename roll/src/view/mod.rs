use crate::State;

use dioxus::prelude::*;
use log::LogView;
use roll::Rollers;

use std::borrow::BorrowMut;
pub mod dioxus_time;
mod log;
mod roll;
pub mod time_observer;

#[component]
pub(crate) fn Body() -> Element {
    use crate::save_url;

    let mut state = use_context::<Signal<State>>();

    let lines = state.read().lines.clone();
    let lines2 = lines.clone();

    rsx!(
        h1 { "Roller" }
        div { class: "bar",
            span { class: "bar-item",
                button { onclick: move |_| { save_url(&state.read().lines) }, "Save to URL" }
            }
            Storage {}
            span { class: "bar-item",
                span { "Load File: " }
                input {
                    r#type: "file",
                    directory: false,
                    accept: ".txt",
                    multiple: false,
                    onchange: move |evt| {
                        async move {
                            if let Some(file_engine) = evt.files() {
                                let files = file_engine.files();
                                for file_name in &files {
                                    if let Some(file) = file_engine.read_file_to_string(file_name).await
                                    {
                                        state.write().borrow_mut().lines = file;
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
        div { class: "row",
            div { class: "column",
                h2 { style: "flex: 0;", "Edit:" }
                span {
                    "Syntax: "
                    a { href: "https://commonmark.org/help/", "Markdown" }
                    " with "
                    a { href: "https://github.com/Geobert/caith?tab=readme-ov-file#syntax",
                        "Caith dice notation"
                    }
                    "."
                }
                span {
                    "Dice notation can be on its own line or in a "
                    i { style: "white-space: nowrap;", "<Roll d=\"dice here\"/>" }
                    " tag."
                }
                textarea {
                    style: "flex-grow: 1;",
                    value: "{lines}",
                    oninput: {
                        move |event: Event<FormData>| {
                            state.write().borrow_mut().lines = event.value();
                        }
                    },
                }
            }
            div { class: "column", id: "Roll",
                Rollers { lines: lines2 }
            }
            div { class: "column", id: "Log", LogView {} }
        }
        div { class: "bar",
            span { class: "bar-item",
                a { href: "https://github.com/Craig-Macomber/game-tools", "Source code" }
            }
            span { class: "bar-item",
                a { href: "/license.html", "Open source usage attributions" }
            }
            span { class: "bar-item",
                a { href: "/known-issues.html", "Known issues" }
            }
            span { class: "bar-item",
                a { href: "/data.html", "Data" }
            }
        }
    )
}

#[component]
#[cfg(target_arch = "wasm32")]
fn Storage() -> Element {
    use crate::{load_storage, save_storage, STORAGE_KEY};

    let mut state = use_context::<Signal<State>>();

    rsx!(
        span { class: "bar-item",
            "Local Storage:"
            button { onclick: move |_| { save_storage(STORAGE_KEY, Some(&state.read().lines)) },
                "Save"
            }
            button {
                onclick: move |_| {
                    let storage = load_storage(STORAGE_KEY);
                    match storage {
                        Some(data) => {
                            state.write().borrow_mut().lines = data;
                        }
                        None => {
                            web_sys::window()
                                .unwrap()
                                .alert_with_message("No data in local storage to load.")
                                .unwrap();
                        }
                    }
                },
                "Load"
            }
            button { onclick: |_| { save_storage(STORAGE_KEY, None) }, "Clear" }
        }
    )
}

#[component]
#[cfg(not(target_arch = "wasm32"))]
fn Storage() -> Element {
    rsx!(
        span { class: "bar-item", "Local storage not supported" }
    )
}
