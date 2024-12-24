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

    let name = state.read().name.clone();

    rsx!(
        div {
            h1 { "Roller" }
            div { class: "bar",
                span { class: "bar-item",
                    span { "Name: " }
                    input {
                        // we tell the component what to render
                        value: "{name}",
                        // and what to do when the value changes
                        oninput: {
                            move |event: Event<FormData>| {
                                state.write().borrow_mut().name = event.value();
                            }
                        },
                    }
                }
                span { class: "bar-item",
                    button { onclick: move |_| { save_default(&name) }, "Save to URL" }
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
                div {
                    class: "column",
                    style: "background-color:#aaa;height:500px;",
                }
                div { class: "column",
                    roll::Roll {}
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
