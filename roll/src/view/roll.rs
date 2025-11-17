use caith::{Command, EvaluatedExpression, Expression, Rollable, Verbosity};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};

use crate::{view::log::LOG, LogItem};

use std::vec;

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
    let roller = Command::parse(&spec);

    match roller {
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
                        LOG.write().log.push(LogItem::new(roll.format(true, Verbosity::Medium)));
                    },
                    "{spec}"
                }
            )
        }
        Err(error) => error,
    }
}

fn validate_roller(spec: &str) -> Result<Command, Element> {
    Command::parse(spec).map_err(|e| roll_error(e, spec))
}

fn roll_error(error: caith::RollError, spec: &str) -> Element {
    rsx!(
        // Not a valid roll, so display as Markdown, but include error from Roller as hover text in case it was intended to be a roll button.
        span { title: "{error}",
            Markdown { src: "{spec}", preserve_html: false }
        }
    )
}

fn get_dice_string(roll: &Box<dyn EvaluatedExpression>) -> String {
    roll.format_history(true, Verbosity::Medium)
}

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in caith dice notation).
 */
#[component]
pub fn Attack(modifier: String, damage_dice: String, damage_fixed: String) -> Element {
    let modifier_roller = match Expression::parse(&modifier) {
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

    let damage_dice_roller = match Expression::parse(&damage_dice) {
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

    let damage_fixed_roller = match Expression::parse(&damage_fixed) {
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
        modifier: &Expression,
        damage_dice: &Expression,
        damage_fixed: &Expression,
    ) {
        let attack_roller = Expression::parse(attack).unwrap();
        let attack_roll: Box<dyn EvaluatedExpression> = attack_roller.roll().unwrap();

        let modifier = modifier.roll().unwrap().total();

        let attack = attack_roll.total() + modifier;

        let damage_dice_roll = damage_dice.roll().unwrap();

        let damage_fixed = damage_fixed.roll().unwrap().total();

        let attack_string = get_dice_string(&attack_roll);
        let damage_string = get_dice_string(&damage_dice_roll);
        let damage_total = damage_dice_roll.total() + damage_fixed;
        let crit = get_crit(&attack_roll);

        LOG.write().log.push(LogItem::new(match crit {
            caith::Critic::No => format!("*To Hit*: **{attack}** = {attack_string} + {modifier} *Damage*: **{damage_total}** = ({damage_string}) + {damage_fixed}"),
            caith::Critic::Min => format!("**Crit Miss** {attack_string}"),
            caith::Critic::Max => {
                 let damage_dice_roll_2 = damage_dice.roll().unwrap();
                let damage_string_2 = get_dice_string(&damage_dice_roll_2);
                let damage_total = damage_total + damage_dice_roll_2.total();
                format!("**Crit** {attack_string} *Damage*: **{damage_total}** = ({damage_string}) + ({damage_string_2}) + {damage_fixed}")
            },
        }));
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
                b { "-" }
            }
            button {
                class: "roll-button",
                onclick: move |_| {
                    roll(regular, &modifier_roller_2, &damage_dice_roller_2, &damage_fixed_roller_2);
                },
                span {
                    b { "1d20 + {modifier}" }
                    " to hit for "
                    b { "{damage_dice} + {damage_fixed}" }
                    " damage"
                }
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
                b { "+" }
            }
        }
    )
}

fn get_crit(r: &Box<dyn EvaluatedExpression>) -> caith::Critic {
    let crit = r.total() == 20.0;
    if crit {
        return caith::Critic::Max;
    }
    let crit = r.total() == 1.0;

    if crit {
        return caith::Critic::Min;
    }

    caith::Critic::No
}
