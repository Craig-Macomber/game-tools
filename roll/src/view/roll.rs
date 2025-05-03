use caith::Roller;
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::{Log, LogItem};

use std::{ops::Deref, vec};

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in caith dice notation).
 */
#[component]
pub(crate) fn ConstantRoll(spec: String) -> Element {
    let mut log = use_context::<Signal<Log>>();

    // Always succeeds: errors are deferred until rolling.
    let roller = Roller::new(&spec).unwrap();

    // Rolled only to see if there is an error.
    let dummy_roll = roller.roll();

    // Empty line case
    if spec.is_empty() {
        return rsx!(
            // Non-breaking 0 width space so span takes up a line
            span { "\u{200b}" }
        );
    }

    match dummy_roll {
        Ok(_) => {
            rsx!(
                div { style: "display: flex;",
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
