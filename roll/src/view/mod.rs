use crate::save_default;
use crate::{Log, State};

use dioxus::prelude::*;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
mod roll;

#[component]
pub(crate) fn Body() -> Element {
    let log = use_context_provider(|| Signal::new(Log::default()));

    let mut state = use_context::<Signal<State>>();

    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);

    let lines = state.read().lines.clone();
    let lines2 = state.read().lines.clone();

    rsx!(
        div {
            h1 { "Roller" }
            div { class: "bar",
                span { class: "bar-item",
                    button { onclick: move |_| { save_default(&lines2) }, "Save to URL" }
                }
                span { class: "bar-item",
                    span { "Load Data: " }
                    input {
                        // tell the input to pick a file
                        r#type: "file",
                        directory: true,
                        // list the accepted extensions
                        // accept: ".txt,.rs",
                        // pick multiple files
                        // multiple: true,
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    filenames.write().push(file_name);
                                }
                            }
                        },
                    }
                }
            }
            div { class: "row",
                div { class: "column",
                    h2 { style: "flex: 0;", "Edit:" }
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
    )
}
