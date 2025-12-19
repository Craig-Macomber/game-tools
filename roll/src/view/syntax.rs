use crate::components::accordion::*;
use dioxus::prelude::*;

#[component]
pub fn Syntax() -> Element {
    rsx! {
        Accordion { allow_multiple_open: true,
            AccordionItem { index: 0, default_open: true,
                AccordionTrigger {
                    div {  h3 { "Syntax" }}
                }
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
                AccordionTrigger {
                    h3 { "Tags" }
                }
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
