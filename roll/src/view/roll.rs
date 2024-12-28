use caith::Roller;
use dioxus::prelude::*;

use crate::Log;

use std::ops::Deref;

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

    match dummy_roll {
        Ok(_) => {
            rsx!(
                div { style: "display: flex;",
                    button {
                        onclick: move |_| {
                            let roll = roller.roll().unwrap();
                            // let message = roll.to_string();
                            if let Some(single) = roll.as_single()  {
                                let message = single.to_string(false);
                                let msg = format!("{spec}: {message}");
                                log.write().log.push(msg);
                            } else {
                                let roll = roll.as_repeated().unwrap();
                                for single in roll.deref() {
                                    let message = single.to_string(false);
                                    let msg = format!("\u{00a0}\u{00a0}\u{00a0}\u{00a0}{message}");
                                    log.write().log.push(msg);
                                }

                                let total = roll.get_total().map_or("".to_owned(), |x| x.to_string());
                                log.write().log.push(format!("{spec}: {total}"));
                            }
                        },
                        "{spec}"
                    }
                }
            )
        }
        Err(d) => rsx!(
            span { title: "{d}", "{spec}" }
        ),
    }
}
mod tests {
    use caith::Roller;
    #[test]
    fn caith_minimal() {
        // This should deterministically roll a 1
        let roller = Roller::new(&"1d1").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let history = numeric.to_string_history();
        let as_string = numeric.to_string(false);

        assert_eq!(numeric.get_total(), 1);
        assert_eq!(as_string, "[1] = 1");
        // Not sure how the history works, maybe this is expected?
        assert_eq!(history, "[1]");
    }

    #[test]
    fn caith_reroll() {
        // This should deterministically roll a 1, then reroll 1
        let roller = Roller::new(&"1d1 r1").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let history = numeric.to_string_history();
        let as_string = numeric.to_string(false);

        assert_eq!(numeric.get_total(), 1);
        assert_eq!(as_string, "[1] = 1");
        // Not sure how the history works, maybe this is expected?
        assert_eq!(history, "[1]");
    }

    #[test]
    fn caith_no_reroll() {
        // This should deterministically roll a 1, then not reroll anything since 1 > 0
        let roller = Roller::new(&"1d1 r0").unwrap();

        let result = roller.roll().unwrap();
        let numeric = result.as_single().unwrap();
        let history = numeric.to_string_history();
        let as_string = numeric.to_string(false);

        // For an unknown reason this roll is producing 0 and not 1.
        assert_eq!(numeric.get_total(), 0);
        // The formatted string output is just "0", which also seems wrong.
        assert_eq!(as_string, "0");
        // The "history" is an empty string. Not sure if this is expected.
        assert_eq!(history, "");
    }
}
