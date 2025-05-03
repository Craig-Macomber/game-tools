use dioxus::prelude::*;
use dioxus_markdown::Markdown;

use crate::LogItem;

use std::vec;

/**
 * Display log item.
 */
#[component]
pub(crate) fn LogItemView(item: LogItem) -> Element {
    rsx!(
        // TODO: concise localized relative time
        // TODO: proper accessible tooltip
        // TODO: some animation to highlight new items (fade in? Use monotonic time?)
        span { title: "{item.timestamp}",
            Markdown { src: "{item.markdown}" }
        }
    )
}
