use caith::Roller;
use dioxus::prelude::*;

use crate::Log;

pub(crate) fn describe_roller(roller: &Roller) -> String {
    let f = format!("{:?}", roller);
    match roller.roll() {
        Ok(_) => f,
        Err(e) => format!("{:?}: {}", roller, e),
    }
}

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
                            let roller_description = describe_roller(&roller);
                            let msg = format!("{roller_description}: {roll}");
                            log.write().log.push(msg);
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
