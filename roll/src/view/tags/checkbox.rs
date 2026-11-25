use dioxus::prelude::*;
use dioxus_markdown::ReadWriteBox;

use crate::view::tags::TagKindData;

pub const TAG: TagKindData = TagKindData {
    tag: "Checkbox",
    short_tag: Some("C"),
    example: r#"<Checkbox value="true"/> or <Checkbox value="false"/>"#,
    description: None,
    component: |lines, props, offset| {
        let value = props
            .get_attribute("value")
            .ok_or(rsx! { "Missing \"value\" attribute." })?;
        let range = (value.range.start + offset)..(value.range.end + offset);
        let state = ReadWriteBox::from_sub_string(lines, range)?;

        Ok(rsx! {
            Checkbox { state }
        })
    },
};

/// A Checkbox
#[component]
fn Checkbox(state: ReadWriteBox<bool>) -> Element {
    let state2 = state.clone();
    rsx! {
        span {
            input {
                r#type: "checkbox", checked: state.read_value(), oninput: move |evt| {
                    if let Ok(new_value) = evt.value().parse::<bool>() {
                        state2.write_value(new_value)
                    }
                }
            }
        }
    }
}
