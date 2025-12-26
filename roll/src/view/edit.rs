use std::borrow::BorrowMut;

use crate::components::button::*;
use crate::components::textarea::Textarea;
use crate::{State, components::accordion::*, save_url};
use dioxus::prelude::*;

#[component]
pub fn Edit(state: Signal<State>) -> Element {
    let lines = state.read().lines;
    rsx! {
        h2 { style: "flex: 0;", "Edit:" }
        Syntax { state }
        Textarea {
            style: "flex-grow: 1;",
            value: "{lines}",
            oninput: {
                move |event: Event<FormData>| {
                    state.write().lines.set(event.value());
                }
            },
        }
    }
}

#[component]
#[cfg(target_arch = "wasm32")]
fn Storage() -> Element {
    use crate::{STORAGE_KEY, load_storage, save_storage};

    let mut state = use_context::<Signal<State>>();

    rsx!(
        span {
            "Local Storage:"
            Button { onclick: move |_| { save_storage(STORAGE_KEY, Some(&state.read().lines.read())) },
                "Save"
            }
            Button {
                onclick: move |_| {
                    let storage = load_storage(STORAGE_KEY);
                    match storage {
                        Some(data) => {
                            state.write().borrow_mut().lines.set(data);
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

#[component]
fn Syntax(state: Signal<State>) -> Element {
    rsx! {
        Accordion { allow_multiple_open: true,
            AccordionItem { index: 0, default_open: true,
                AccordionTrigger {
                    div {
                        h3 { "Save and Load" }
                    }
                }
                AccordionContent {
                    span {
                        Button { onclick: move |_| { save_url(&state.read().lines.read()) },
                            "Save to URL"
                        }
                    }
                    Storage {}
                    span {
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
                                            state.write().borrow_mut().lines.set(file);
                                        }
                                    }
                                }
                            },
                        }

                    }
                }
            }
            AccordionItem { index: 1,
                AccordionTrigger {
                    div {
                        h3 { "Syntax" }
                    }
                }
                AccordionContent {
                    span {
                        a { href: "https://commonmark.org/help/", "Markdown" }
                        " with "
                        a { href: "https://github.com/Craig-Macomber/game-tools/blob/main/dicey/README.md#syntax",
                            "Dicey dice notation"
                        }
                        "."
                    }
                    span {
                        "Dice notation can be on its own line or in a "
                        i { style: "white-space: nowrap;", "<Roll d=\"dice here\"/>" }
                        " tag."
                    }
                }
            }
            AccordionItem { index: 2,
                AccordionTrigger {
                    h3 { "Tags" }
                }
                AccordionContent {
                    span {
                        "Roll: "
                        i { style: "white-space: nowrap;", "<Roll d=\"dice here\"/>" }
                    }
                    span {
                        "Counter: "
                        i { style: "white-space: nowrap;", "<Counter initial=\"20\"/>" }
                    }
                    span {
                        "Attack: "
                        i { style: "white-space: nowrap;", r#"<A m="5" d="2d6 + 1d8" f="2"/>"# }
                    }
                }
            }
        }
    }
}
