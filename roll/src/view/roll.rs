use caith::{RollError, Roller};
use dioxus::prelude::*;

use crate::Log;

#[component]
pub(crate) fn Roll() -> Element {
    let mut log = use_context::<Signal<Log>>();

    let mut roll = use_signal(|| "1d20".to_owned());

    let roller = Roller::new(&roll.read());
    let roll_view = view_roll(&roller);

    rsx!(
        form {
            style: "display: flex;",
            onsubmit: {
                let roller = Roller::new(&roll.read());
                move |event| {
                    log::info!("Rolled! {event:?}");
                    match &roller {
                        Ok(d) => {
                            if let Ok(d) = d.roll() {
                                let msg = format!("{roll_view}: {}", d);
                                log.write().log.push(msg);
                            }
                        }
                        Err(_) => {}
                    }
                }
            },
            input {
                style: "flex-grow: 1;",
                name: "message",
                value: "{roll}",
                oninput: move |event| { roll.set(event.value()) },
            }
            span { " --> {roll_view}" }
            input { r#type: "submit", value: "Roll" }
        }
    )
}

pub(crate) fn view_roll(roller: &Result<Roller, RollError>) -> String {
    match roller {
        Ok(d) => {
            let f = format!("{:?}", d);
            match d.roll() {
                Ok(_) => f,
                Err(e) => format!("{:?}: {}", d, e),
            }
        }
        Err(d) => format!("Parse Error: {d}"),
    }
}
