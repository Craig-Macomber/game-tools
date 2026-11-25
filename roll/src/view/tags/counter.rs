use dioxus::prelude::*;
use dioxus_markdown::ReadWriteBox;

use crate::components::button::*;
use crate::view::tags::TagKindData;

use std::vec;

pub const TAG: TagKindData = TagKindData {
    tag: "Counter",
    short_tag: None,
    example: "<Counter value=\"20\"/>",
    description: None,
    component: |lines, props, offset| {
        let value = props
            .get_attribute("value")
            .ok_or(rsx! { "Missing \"value\" attribute." })?;
        let range = (value.range.start + offset)..(value.range.end + offset);
        let count = ReadWriteBox::from_sub_string(lines, range)?;
        Ok(rsx! {
            Counter { count }
        })
    },
};

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
