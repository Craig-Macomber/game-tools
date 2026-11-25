use dicey::{EvaluatedExpression, Expression, FancyFormat, Rollable, Verbosity};
use dioxus::prelude::*;

use crate::LogItem;
use crate::components::button::*;
use crate::view::log::LOG;
use crate::view::tags::roll::roll_error;
use crate::view::tags::{Constants, TagKindData};

use std::vec;

pub const TAG: TagKindData = TagKindData {
    tag: "Attack",
    short_tag: Some("A"),
    example: r#"<A m="5" d="2d6 + 1d8" f="2"/>"#,
    description: Some(
        "m = attack bonus added to d20 roll; d = dice expression for damage, doubles for crits; f = fixed component not impacted by crits, added to damage",
    ),
    component: |_lines, props, _offset| {
        Ok(rsx! {
            Attack {
                modifier: props.get("m").unwrap_or("Invalid".to_string()),
                damage_dice: props.get("d").unwrap_or("Invalid".to_string()),
                damage_fixed: props.get("f").unwrap_or("Invalid".to_string()),
            }
        })
    },
};

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

fn get_dice_string(roll: &dyn EvaluatedExpression) -> String {
    roll.format_history(true, Verbosity::Medium)
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
