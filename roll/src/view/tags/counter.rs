use dicey::{Expression, FancyFormat, Rollable, Verbosity};
use dioxus::prelude::*;
use dioxus_markdown::ReadWriteBox;

use web_framework_markdown::MdComponentAttribute;

use crate::LogItem;
use crate::components::button::*;
use crate::view::log::LOG;
use crate::view::tags::roll::roll_error;
use crate::view::tags::{Constants, TagKindData};

pub const TAG: TagKindData = TagKindData {
    tag: "Counter",
    short_tag: None,
    example: r#"<Counter value="20"/> or <Counter value="20" default="2d10+10"/>"#,
    description: Some(
        r#""default" is an optional dice expression which will provide an option to reset the count to that value"#,
    ),
    component: |lines, props, offset| {
        let value = props
            .get_attribute("value")
            .ok_or(rsx! { "Missing \"value\" attribute." })?;
        let range = (value.range.start + offset)..(value.range.end + offset);
        let count = ReadWriteBox::from_sub_string(lines, range)?;

        let default = props.get_attribute("default");

        Ok(rsx! {
            Counter { count, default }
        })
    },
};

/// A counter who's current count is stored in the document.
#[component]
fn Counter(count: ReadWriteBox<i32>, default: Option<MdComponentAttribute>) -> Element {
    let mut count2 = count.clone();
    let count3 = count.clone();
    let count4 = count.clone();
    let mut delta = use_signal(|| "1".to_string());

    let constants = use_context::<Constants>();
    let default = default.map(|spec| validate_roller(&spec.value, &constants));

    let delta_roller = validate_roller(&delta.read(), &constants);

    rsx! {
        span {
            input {
                r#type: "number", name: "quantity", value: "{count3}", oninput: move |evt| {
                    if let Ok(new_value) = evt.value().parse::<i32>() {
                        count3.write_value(new_value)
                    }
                }
            }
            " ("
            input {
                name: "delta", value: "{delta}", oninput: move |evt| {
                    delta.set(evt.value())
                }
            }
            match delta_roller {
                Ok(roller) => {
                    let text = roller.format(false, Verbosity::Medium);
                    let roller2 = roller.clone();
                    let roller3 = roller.clone();
                    rsx!(
                        Button { onclick: move |_| count2 -= logged_roll(&roller2, "decrement counter").unwrap_or(0), "- {text}" }
                        "/"
                        Button { onclick: move |_| count += logged_roll(&roller3, "increment counter").unwrap_or(0), "+ {text}" }
                    )
                }
                // TODO: this case line wraps when it shouldn't
                Err(error) => error,
            }


            if let Some(default) = default {
                " "
                match default {
                   // TODO: share some of this logic with Roll tag's Roll component
                    Ok(roller) => {
                        let text = roller.format(false, Verbosity::Medium);
                        rsx!(
                            Button {
                                title: roller.format(false, Verbosity::Verbose),
                                onclick: move |_| {
                                    if let Some(int_total) = logged_roll(&roller, "reset to default") {
                                        count4.write_value(int_total);
                                    };
                                },
                                "Reset to {text}"
                            }
                        )
                    }
                    Err(error) => error,
                }
            }
            ")"
        }
    }
}

fn validate_roller(spec: &str, constants: &Constants) -> Result<Expression, Element> {
    Expression::parse_with_variables(spec, &constants.values).map_err(|e| roll_error(e, spec))
}

fn logged_roll(roller: &Expression, operation: &'static str) -> Option<i32> {
    let roll = roller.roll();
    match roll {
        Ok(roll) => {
            let total = roll.total();
            let int_total: i32 = total as i32;

            LOG.write().log.push(LogItem::new(format!(
                "Counter {operation} {int_total} ({0})",
                roll.format(true, Verbosity::Medium)
            )));
            Some(int_total)
        }
        Err(err) => {
            LOG.write().log.push(LogItem::new(format!(
                "Error performing {operation} on counter: {0}",
                format!("{err}")
            )));
            None
        }
    }
}
