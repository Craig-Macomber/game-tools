use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        accordion::Accordion {
            class: "accordion",
            // width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn Syntax() -> Element {
    rsx! {
        Accordion {
            AccordionItem { index: 0, default_open: true,
                AccordionTrigger { "Syntax" }
                AccordionContent {
                    span {
                        a { href: "https://commonmark.org/help/", "Markdown" }
                        " with "
                        a { href: "https://github.com/Geobert/caith?tab=readme-ov-file#syntax",
                            "Caith dice notation"
                        }
                        "."
                    }
                    span {
                        "Dice notation can be on its own line or in a "
                        i { style: "white-space: nowrap;", "<Roll d=\"dice here\"/>" }
                        " tag."
                    }
                }
            }
            AccordionItem { index: 2,
                AccordionTrigger { "Tags" }
                AccordionContent {
                    span {
                        "Roll: "
                        i { style: "white-space: nowrap;", "<Roll d=\"dice here\"/>" }
                    }
                    span {
                        "Counter: "
                        i { style: "white-space: nowrap;", "<Counter initial=\"20\"/>" }
                    }
                    span {
                        "Attack: "
                        i { style: "white-space: nowrap;", r#"<A m="5" d="2d6 + 1d8" f="2"/>"# }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: "accordion-item",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "accordion-trigger",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            svg {
                class: "accordion-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            class: "accordion-content",
            // style: "--collapsible-content-width: 140px",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
