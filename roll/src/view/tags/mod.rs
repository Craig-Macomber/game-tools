use dicey::Expression;
use dioxus::prelude::*;
use dioxus_markdown::{ComponentCreationError, MdComponentProps};

use std::collections::HashMap;
use std::rc::Rc;

mod attack;
mod counter;
mod roll;

type ComponentFactory = fn(
    lines: Signal<String>,
    props: MdComponentProps,
    offset: usize,
) -> Result<Element, ComponentCreationError>;

#[derive(Debug, Clone)]
pub(crate) struct TagKindData {
    pub tag: &'static str,
    pub short_tag: Option<&'static str>,
    pub example: &'static str,
    pub description: Option<&'static str>,
    pub component: ComponentFactory,
}

pub const TAG_KINDS: &'static [TagKindData] = &[roll::TAG, counter::TAG, attack::TAG];

#[derive(Default, Clone)]
pub struct Constants {
    pub values: Rc<HashMap<String, Expression>>,
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
