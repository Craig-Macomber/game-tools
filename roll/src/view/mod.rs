use crate::State;
use crate::view::edit::Edit;

use dioxus::prelude::*;
use log::LogView;
use roll::Rollers;

pub mod dioxus_time;
mod edit;
mod log;
mod roll;
pub mod time_observer;

#[component]
pub(crate) fn Body() -> Element {
    let state = use_context::<Signal<State>>();
    let lines = state.read().lines;

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
                Edit { state }
            }
            div { class: "column", id: "Roll",
                Rollers { lines }
            }
            div { class: "column", id: "Log", LogView {} }
        }
    )
}
