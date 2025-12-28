use dicey::{Command, EvaluatedExpression, Expression, FancyFormat, Rollable, Variable, Verbosity};
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown, ReadWriteBox};
use subslice_offset::SubsliceOffset;

use crate::components::button::*;
use crate::{LogItem, view::log::LOG};

use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

/**
 * Rendered markdown results with inline rollers that put their result in "Log".
 */
#[component]
pub(crate) fn Rollers(lines: Signal<String>) -> Element {
    let mut markdown: Vec<Element> = vec![];
    let lines_holder = lines.read().clone();
    let lines_slice = &lines_holder[..];

    let mut constants = HashMap::default();

    for line in lines_slice.lines() {
        let offset = lines_slice.subslice_offset(line).unwrap();
        let roller = try_roller(line, &constants);
        let (line, negative_offset) = match roller {
            Some(_) => (format!("<R d=\"{line}\"/>"), 6),
            None => (line.to_string(), 0),
        };

        if let Ok(con) = Variable::parse_with_variables(&line, &constants) {
            let message = format!("Constant: {} = {}", &con.identifier, &con.expression);
            markdown.push(rsx!(
                p { "{message}" }
            ));
            constants.insert(con.identifier, con.expression);
            continue;
        }

        let mut components = CustomComponents::new();

        components.register("Counter", move |props| {
            let value = props
                .get_attribute("value")
                .ok_or(rsx! { "Missing \"value\" attribute." })?;
            let range = (value.range.start + offset - negative_offset)
                ..(value.range.end + offset - negative_offset);
            let count = ReadWriteBox::from_sub_string(lines, range)?;
            Ok(rsx! {
                Counter { count }
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

        let md = rsx!(Markdown {
            src: line.clone(),
            components,
            preserve_html: false
        });

        markdown.push(rsx!(
            p {
                ConstantsProvider {
                    children: md,
                    constants: Constants {
                        values: constants.clone().into(),
                    },
                }
            }
        ));
    }

    rsx!(
        h2 { "Roll:" }
        div { id: "Roll-Content",
            for item in markdown {
                {item}
            }
        }
    )
}

#[component]
fn ConstantsProvider(children: Element, constants: Constants) -> Element {
    use_context_provider(|| constants);
    children
}

#[derive(Default, Clone)]
struct Constants {
    values: Rc<HashMap<String, Expression>>,
}

impl PartialEq for Constants {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.values, &other.values)
        //     if self.values.len() != other.values.len() {
        //         return false;
        //     }

        //     for (k, v) in &self.values {
        //         match other.values.get(k) {
        //             Some(occupied_entry) => {
        //                 match (occupied_entry, v) {
        //                     (Ok(a), Ok(b)) => {
        //                         if  a != b { return false; }
        //                     },
        //                     (Ok(_), Err(_)) => return false,
        //                     (Err(_), Ok(_)) => return false,
        //                     (Err(a), Err(b)) => {
        //                         if a != b { return false; }
        //                     },
        //                 }
        //             },
        //             None =>  return false,
        //         }
        //     }

        //     return true;
    }
}

pub fn try_roller(spec: &str, constants: &HashMap<String, Expression>) -> Option<String> {
    let roller = Command::parse_with_variables(spec, constants);

    match roller {
        Ok(_) => Some(spec.to_string()),
        Err(_) => None,
    }
}

/// A counter who's current count is stored in the document.
#[component]
fn Counter(count: ReadWriteBox<i32>) -> Element {
    let mut count2 = count.clone();
    rsx! {
        span {
            Button { onclick: move |_| count2 -= 1, "-" }
            "{count}"
            Button { onclick: move |_| count += 1, "+" }
        }
    }
}

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

fn roll_error(error: dicey::RollError, spec: &str) -> Element {
    rsx!(
        // Not a valid roll, so display as Markdown, but include error from Roller as hover text in case it was intended to be a roll button.
        span { title: "{error}",
            Markdown { src: "{spec}", preserve_html: false }
        }
    )
}

fn get_dice_string(roll: &dyn EvaluatedExpression) -> String {
    roll.format_history(true, Verbosity::Medium)
}

/**
 * Display text, or a roll button depending on if string is a valid roll specification (in dicey dice notation).
 */
#[component]
pub fn Attack(modifier: String, damage_dice: String, damage_fixed: String) -> Element {
    let constants = &use_context::<Constants>().values;
    let modifier_roller = match Expression::parse_with_variables(&modifier, constants) {
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

    let damage_dice_roller = match Expression::parse_with_variables(&damage_dice, constants) {
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

    let damage_fixed_roller = match Expression::parse_with_variables(&damage_fixed, constants) {
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
        let s = match roll_inner(attack, modifier, damage_dice, damage_fixed) {
            Ok(s) => s,
            Err(e) => format!("{e}"),
        };

        LOG.write().log.push(LogItem::new(s));
    }

    fn roll_inner(
        attack: &str,
        modifier: &Expression,
        damage_dice: &Expression,
        damage_fixed: &Expression,
    ) -> dicey::Result<String> {
        let attack_roller = Expression::parse(attack)?;
        let attack_roll: Box<dyn EvaluatedExpression> = attack_roller.roll()?;

        let modifier = modifier.roll()?.total();

        let attack = attack_roll.total() + modifier;

        let damage_dice_roll = damage_dice.roll()?;

        let damage_fixed = damage_fixed.roll()?.total();

        let attack_string = get_dice_string(&*attack_roll);
        let damage_string = get_dice_string(&*damage_dice_roll);
        let damage_total = damage_dice_roll.total() + damage_fixed;
        let crit = get_crit(&*attack_roll);

        Ok(match crit {
            Critic::No => format!(
                "*To Hit*: **{attack}** = {attack_string} + {modifier} *Damage*: **{damage_total}** = {damage_string} + {damage_fixed}"
            ),
            Critic::Min => format!("**Crit Miss** {attack_string}"),
            Critic::Max => {
                let damage_dice_roll_2 = damage_dice.roll()?;
                let damage_string_2 = get_dice_string(&*damage_dice_roll_2);
                let damage_total = damage_total + damage_dice_roll_2.total();
                format!(
                    "**Crit** {attack_string} *Damage*: **{damage_total}** = {damage_string} + {damage_string_2} + {damage_fixed}"
                )
            }
        })
    }

    let modifier_text = modifier_roller.format(false, Verbosity::Medium);
    let damage_text = damage_dice_roller.format(false, Verbosity::Medium);
    let damage_fixed_text = damage_fixed_roller.format(false, Verbosity::Medium);
    let title = format!(
        "1d20 + {} to hit for {} + {} damage",
        modifier_roller.format(false, Verbosity::Verbose),
        damage_dice_roller.format(false, Verbosity::Verbose),
        damage_fixed_roller.format(false, Verbosity::Verbose)
    );

    let modifier_roller_2 = modifier_roller.clone();
    let modifier_roller_3 = modifier_roller.clone();
    let damage_dice_roller_2 = damage_dice_roller.clone();
    let damage_dice_roller_3 = damage_dice_roller.clone();
    let damage_fixed_roller_2 = damage_fixed_roller.clone();
    let damage_fixed_roller_3 = damage_fixed_roller.clone();
    rsx!(
        span {
            Button {
                title: disadvantage,
                onclick: move |_| {
                    roll(disadvantage, &modifier_roller, &damage_dice_roller, &damage_fixed_roller);
                },
                b { "-" }
            }
            Button {
                title,
                onclick: move |_| {
                    roll(regular, &modifier_roller_2, &damage_dice_roller_2, &damage_fixed_roller_2);
                },
                span {
                    b { "1d20 + {modifier_text}" }
                    " to hit for "
                    b { "{damage_text} + {damage_fixed_text}" }
                    " damage"
                }

            }
            Button {
                title: advantage,
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

fn get_crit(r: &dyn EvaluatedExpression) -> Critic {
    let crit = r.total() == 20.0;
    if crit {
        return Critic::Max;
    }
    let crit = r.total() == 1.0;

    if crit {
        return Critic::Min;
    }

    Critic::No
}

/// Used to mark a dice roll if its result is a critic.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Critic {
    /// Normal result
    No,
    /// Minimum reached
    Min,
    /// Maximum reached
    Max,
}
