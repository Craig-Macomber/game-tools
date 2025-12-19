use crate::State;
use crate::components::button::*;

use dioxus::prelude::*;
use log::LogView;
use roll::Rollers;

use std::borrow::BorrowMut;
pub mod dioxus_time;
mod log;
mod roll;
mod syntax;
pub mod time_observer;

use syntax::Syntax;

#[component]
pub(crate) fn Body() -> Element {
    use crate::save_url;

    let mut state = use_context::<Signal<State>>();

    let lines = state.read().lines.clone();
    let lines2 = lines.clone();

    rsx!(
        div { class: "bar",
            span { class: "bar-item",
                h1 { "Roller" }
            }
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

        div { class: "row",
            div { class: "column",
                h2 { style: "flex: 0;", "Edit:" }
                div { class: "bar",
                    span { class: "bar-item",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| { save_url(&state.read().lines) },
                            "Save to URL"
                        }
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
                                    for file in evt.files() {
                                        if let Ok(file) = file.read_string().await {
                                            state.write().borrow_mut().lines = file;
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
                Syntax {}

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
    )
}

#[component]
#[cfg(target_arch = "wasm32")]
fn Storage() -> Element {
    use crate::{STORAGE_KEY, load_storage, save_storage};

    let mut state = use_context::<Signal<State>>();

    rsx!(
        span { class: "bar-item",
            "Local Storage:"
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| { save_storage(STORAGE_KEY, Some(&state.read().lines)) },
                "Save"
            }
            Button {
                variant: ButtonVariant::Secondary,
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
            Button {
                variant: ButtonVariant::Destructive,
                onclick: |_| { save_storage(STORAGE_KEY, None) },
                "Clear"
            }
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
