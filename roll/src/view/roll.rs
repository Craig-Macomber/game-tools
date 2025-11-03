use caith::{Roller, SingleRoller};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

use crate::{view::log::LOG, LogItem};

use std::{ops::Deref, vec};

/**
 * Rendered markdown results with inline rollers that put their result in "Log".
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
            Roll { spec: props.get("d").unwrap_or("Invalid".to_string()) }
        })
    });

    components.register("R", |props| {
        Ok(rsx! {
            Roll { spec: props.get("d").unwrap_or("Invalid".to_string()) }
        })
    });

    components.register("A", |props| {
        Ok(rsx! {
            Attack {
                modifier: props.get("m").unwrap_or("Invalid".to_string()),
                damage_dice: props.get("d").unwrap_or("Invalid".to_string()),
                damage_fixed: props.get("f").unwrap_or("Invalid".to_string()),
            }
        })
    });

    let mut markdown: Vec<String> = vec![];
    for line in lines.lines() {
        let roller = try_roller(line);
        match roller {
            Some(_) => markdown.push(format!("<R d=\"{line}\"/>")),
            None => markdown.push(line.to_string()),
        }
    }

    rsx!(
        h2 { "Roll:" }
        div { id: "Roll-Content",
            Markdown {
                src: markdown.join("\n"),
                components,
                preserve_html: false,
            }
        }
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
        Err(_) => None,
    }
}

#[component]
fn Counter(initial: i32) -> Element {
    let mut count = use_signal(|| initial);

    rsx! {
        span {
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
    match validate_roller(&spec) {
        Ok(roller) => {
            rsx!(
                button {
                    class: "roll-button",
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
                        LOG.write().log.push(LogItem::new(log_lines.join("\n")));
                    },
                    "{spec}"
                }
            )
        }
        Err(error) => error,
    }
}

fn validate_roller(spec: &str) -> Result<Roller, Element> {
    // Always succeeds: errors are deferred until rolling.
    let roller = Roller::new(spec).unwrap();

    // Rolled only to see if there is an error.
    let dummy_roll = roller.roll();

    match dummy_roll {
        Ok(_) => Ok(roller),
        Err(error) => Err(roll_error(error, spec)),
    }
}

fn roll_error(error: caith::RollError, spec: &str) -> Element {
    rsx!(
        // Not a valid roll, so display as Markdown, but include error from Roller as hover text incase it was intended to be a roll button.
        span { title: "{error}",
            Markdown { src: "{spec}", preserve_html: false }
        }
    )
}

fn get_dice_string(roll: &caith::SingleRollResult) -> String {
    let s: Vec<String> = roll.get_history().iter().map(|h| h.to_string()).collect();
    s.join(" ")
}

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in caith dice notation).
 */
#[component]
pub fn Attack(modifier: String, damage_dice: String, damage_fixed: String) -> Element {
    let modifier_roller = match SingleRoller::new(&modifier) {
        Ok(roller) => roller,
        Err(error) => {
            return rsx!(
                span {
                    "Invalid modifier specification"
                    {roll_error(error, &modifier)}
                }
            );
        }
    };

    let damage_dice_roller = match SingleRoller::new(&damage_dice) {
        Ok(roller) => roller,
        Err(error) => {
            return rsx!(
                span {
                    "Invalid damage dice specification"
                    {roll_error(error, &damage_dice)}
                }
            );
        }
    };

    let damage_fixed_roller = match SingleRoller::new(&damage_fixed) {
        Ok(roller) => roller,
        Err(error) => {
            return rsx!(
                span {
                    "Invalid damage fixed specification"
                    {roll_error(error, &damage_fixed)}
                }
            );
        }
    };

    let regular = "1d20";
    let advantage = "2d20K1";
    let disadvantage = "2d20k1";

    fn roll(
        attack: &str,
        modifier: &SingleRoller,
        damage_dice: &SingleRoller,
        damage_fixed: &SingleRoller,
    ) {
        let attack_roller = SingleRoller::new(attack).unwrap();
        let attack_roll = attack_roller.roll();

        let modifier = modifier.roll().get_total();

        let attack = attack_roll.get_total() + modifier;

        let damage_dice_roll = damage_dice.roll();

        let damage_fixed = damage_fixed.roll().get_total();

        let attack_string = get_dice_string(&attack_roll);
        let damage_string = get_dice_string(&damage_dice_roll);
        let damage_total = damage_dice_roll.get_total() + damage_fixed;

        LOG.write().log.push(LogItem::new(
            if attack_roll.get_total() == 20 {
                let damage_dice_roll_2 = damage_dice.roll();
                let damage_string_2 = get_dice_string(&damage_dice_roll_2);
                let damage_total = damage_total + damage_dice_roll_2.get_total();
                format!("**Crit** {attack_string} *Damage*: **{damage_total}** = ({damage_string}) + ({damage_string_2}) + {damage_fixed}")
            } else if  attack_roll.get_total() == 1 {
                format!("**Crit Miss** {attack_string}")
            } else {
                format!("*To Hit*: **{attack}** = {attack_string} + {modifier} *Damage*: **{damage_total}** = ({damage_string}) + {damage_fixed}")
            }
        ));
    }

    let modifier_roller_2 = modifier_roller.clone();
    let modifier_roller_3 = modifier_roller.clone();
    let damage_dice_roller_2 = damage_dice_roller.clone();
    let damage_dice_roller_3 = damage_dice_roller.clone();
    let damage_fixed_roller_2 = damage_fixed_roller.clone();
    let damage_fixed_roller_3 = damage_fixed_roller.clone();
    rsx!(
        span {
            button {
                class: "roll-button",
                onclick: move |_| {
                    roll(disadvantage, &modifier_roller, &damage_dice_roller, &damage_fixed_roller);
                },
                b {"-"}
            }
            button {
                class: "roll-button",
                onclick: move |_| {
                    roll(regular, &modifier_roller_2, &damage_dice_roller_2, &damage_fixed_roller_2);
                },
                span {b {"1d20 + {modifier}"}" to hit for " b{"{damage_dice} + {damage_fixed}"}" damage"}
            }
            button {
                class: "roll-button",
                onclick: move |_| {
                    roll(
                        advantage,
                        &modifier_roller_3,
                        &damage_dice_roller_3,
                        &damage_fixed_roller_3,
                    );
                },
                b {"+"}
            }
        }
    )
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
