use dicey::Variable;
use dioxus::prelude::*;
use dioxus_markdown::{CustomComponents, Markdown};
use subslice_offset::SubsliceOffset;

use crate::view::tags::{Constants, TAG_KINDS, try_roller};

use std::collections::HashMap;
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
        // Offset of this line withing `lines`
        let offset = lines_slice.subslice_offset(line).unwrap();
        let roller = try_roller(line, &constants);

        // Try special case of implicit tag, which if done,
        // // wraps the line with the implicit tag, and needs the additional offset.
        let (line, negative_offset) = match roller {
            Some(_) => (format!("<R d=\"{line}\"/>"), 6),
            None => (line.to_string(), 0),
        };
        let offset = offset - negative_offset;

        if let Ok(con) = Variable::parse_with_variables(&line, &constants) {
            let message = format!("Constant: {} = {}", &con.identifier, &con.expression);
            markdown.push(rsx!(
                p { "{message}" }
            ));
            constants.insert(con.identifier, con.expression);
            continue;
        }

        let mut components = CustomComponents::new();

        for kind in TAG_KINDS {
            components.register(&kind.tag, move |props| {
                (kind.component)(lines, props, offset)
            });
            if let Some(short_tag) = &kind.short_tag {
                components.register(short_tag, move |props| {
                    (kind.component)(lines, props, offset)
                });
            }
        }

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
