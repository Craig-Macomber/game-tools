use dicey::{Command, FancyFormat, Rollable, Verbosity};
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::LogItem;
use crate::components::button::*;
use crate::view::log::LOG;
use crate::view::tags::{Constants, TagKindData};

use std::vec;

pub const TAG: TagKindData = TagKindData {
    tag: "Roll",
    short_tag: Some("R"),
    example: "<Roll d=\"dice here\"/>",
    description: None,
    component: |_lines, props, _offset| {
        Ok(rsx! {
            Roll { spec: props.get("d").unwrap_or("Invalid".to_string()) }
        })
    },
};

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in dicey dice notation).
 */
#[component]
pub fn Roll(spec: String) -> Element {
    let constants = use_context::<Constants>();
    match validate_roller(&spec, constants) {
        Ok(roller) => {
            let text = roller.format(false, Verbosity::Medium);
            rsx!(
                Button {
                    title: roller.format(false, Verbosity::Verbose),
                    onclick: move |_| {
                        let roll = roller.roll();
                        let message = match roll {
                            Ok(roll) => roll.format(true, Verbosity::Medium),
                            Err(err) => format!("{err}"),
                        };
                        LOG.write().log.push(LogItem::new(message));
                    },
                    "{text}"
                }
            )
        }
        Err(error) => error,
    }
}

fn validate_roller(spec: &str, constants: Constants) -> Result<Command, Element> {
    Command::parse_with_variables(spec, &constants.values).map_err(|e| roll_error(e, spec))
}

#[component]
fn RollError(error: dicey::RollError, spec: String) -> Element {
    roll_error(error, &spec)
}

pub fn roll_error(error: dicey::RollError, spec: &str) -> Element {
    rsx!(
        // Not a valid roll, so display as Markdown, but include error from Roller as hover text in case it was intended to be a roll button.
        span { title: "{error}",
            Markdown { src: "{spec}", preserve_html: false }
        }
    )
}
