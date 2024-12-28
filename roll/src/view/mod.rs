use crate::{load_storage, save_storage, save_url};
use crate::{Log, State};

use dioxus::prelude::*;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
mod roll;

#[component]
pub(crate) fn Body() -> Element {
    let log = use_context_provider(|| Signal::new(Log::default()));

    let mut state = use_context::<Signal<State>>();

    let lines = state.read().lines.clone();

    rsx!(
        div { class: "container",
            h1 { "Roller" }
            div { class: "bar",
                span { class: "bar-item",
                    button { onclick: move |_| { save_url(&state.read().lines) }, "Save to URL" }
                }
                span { class: "bar-item",
                    "Local Storage:"
                    button { onclick: move |_| { save_storage(Some(&state.read().lines)) },
                        "Save"
                    }
                    button {
                        onclick: move |_| {
                            let storage = load_storage();
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
                    button { onclick: |_| { save_storage(None) }, "Clear" }
                }
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
                    a { href: "https://github.com/Geobert/caith?tab=readme-ov-file#syntax",
                        "Syntax"
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
                div { class: "column", style: "background-color:#aaa;",
                    h2 { "Roll:" }
                    for line in lines.lines() {
                        roll::ConstantRoll { spec: line }
                    }
                }
                div { class: "column",
                    h2 { "Log:" }
                    ul {
                        for message in log.read().borrow().log.iter().rev() {
                            li { "{message}" }
                        }
                    }
                }
            }
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
        }
    )
}
