use caith::Roller;
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

use crate::{Log, LogItem};

use std::{ops::Deref, vec};

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in caith dice notation).
 */
#[component]
pub(crate) fn Rollers(lines: String) -> Element {
    let mut components = CustomComponents::new();

    components.register("Counter", |props| {
        Ok(rsx! {
            Counter { initial: props.get_parsed_optional("initial")?.unwrap_or(0) }
        })
    });

    components.register("Roll", |props| {
        Ok(rsx! {
            Roll { spec: props.get("src").unwrap_or("Invalid".to_string()) }
        })
    });

    let mut markdown: Vec<String> = vec![];
    for line in lines.lines() {
        let roller = try_roller(line);
        match roller {
            Some(r) => markdown.push(format!("<Roll src=\"{line}\"/>")),
            None => markdown.push(line.to_string()),
        }
    }

    rsx!(
        h2 { "Roll:" }
        Markdown { src: markdown.join("\n"), components }
    )
}

pub fn try_roller(spec: &str) -> Option<String> {
    // Always succeeds: errors are deferred until rolling.
    let roller = Roller::new(&spec).unwrap();

    // Rolled only to see if there is an error.
    let dummy_roll = roller.roll();

    // Empty line case
    if spec.is_empty() {
        return None;
    }

    match dummy_roll {
        Ok(_) => Some(spec.to_string()),
        Err(d) => None,
    }
}

#[component]
fn Counter(initial: i32) -> Element {
    let mut count = use_signal(|| initial);

    rsx! {
        div {
            button { onclick: move |_| count -= 1, "-" }
            "{count}"
            button { onclick: move |_| count += 1, "+" }
        }
    }
}

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in caith dice notation).
 */
#[component]
pub fn Roll(spec: String) -> Element {
    let mut log = use_context::<Signal<Log>>();

    // Always succeeds: errors are deferred until rolling.
    let roller = Roller::new(&spec).unwrap();

    // Rolled only to see if there is an error.
    let dummy_roll = roller.roll();

    match dummy_roll {
        Ok(_) => {
            rsx!(
                button {
                    onclick: move |_| {
                        let roll = roller.roll().unwrap();
                        let mut log_lines = vec![];
                        if let Some(single) = roll.as_single() {
                            let message = single.to_string(true);
                            let msg = format!("{spec}: {message}");
                            log_lines.push(msg);
                        } else {
                            let roll = roll.as_repeated().unwrap();
                            for single in roll.deref() {
                                let message = single.to_string(true);
                                let msg = format!("  - {message}");
                                log_lines.push(msg);
                            }
                            let total = roll.get_total().map_or("".to_owned(), |x| x.to_string());
                            log_lines.push(format!("\n{spec}: **{total}**"));
                        }
                        log.write().log.push(LogItem::new(log_lines.join("\n")));
                    },
                    "{spec}"
                }
            )
        }
        Err(d) => rsx!(
            // Not a valid roll, so display as Markdown, but include error from Roller as hover text incase it was intended to be a roll button.
            span { title: "{d}",
                Markdown { src: "{spec}" }
            }
        ),
    }
}

#[cfg(test)]
mod tests {

    // Regression tests for https://github.com/Geobert/caith/issues/5
    use caith::Roller;
    #[test]
    fn caith_minimal() {
        // This should deterministically roll a 1
        let roller = Roller::new(&"1d1").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let as_string = numeric.to_string(false);

        assert_eq!(numeric.get_total(), 1);
        assert_eq!(as_string, "[1] = 1");
    }

    #[test]
    fn caith_reroll() {
        // This should deterministically roll a 1, then reroll 1
        let roller = Roller::new(&"1d1 r1").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let as_string = numeric.to_string(false);

        assert_eq!(numeric.get_total(), 1);
        assert_eq!(as_string, "[1 -> 1] -> [1] = 1");
    }

    #[test]
    fn caith_no_reroll() {
        // This should deterministically roll a 1, then not reroll anything since 1 > 0
        let roller = Roller::new(&"1d1 r0").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let as_string = numeric.to_string(false);

        assert_eq!(numeric.get_total(), 1);
        assert_eq!(as_string, "[1] = 1");
    }
}
