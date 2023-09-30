// Converts non-eccentric <ellipse>s to <circle>s.

use swc_core::common::DUMMY_SP;
use swc_xml_ast::*;
use swc_xml_visit::{VisitMut, VisitMutWith};
use serde::Deserialize;

struct Visitor {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {}

impl Default for Params {
    fn default() -> Self {
        Self {}
    }
}

impl Visitor {
    fn new() -> Self {
        Self {}
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, n: &mut Element) {
        if n.tag_name.to_string() == "ellipse" {
            let mut rx = "0".to_string();
            let mut ry = "0".to_string();
            let mut new_attributes = vec![];
            for attr in n.attributes.clone() {
                if attr.name.to_string() == "rx" {
                    if let Some(value) = attr.value.clone() {
                        rx = value.to_string();
                    }
                } else if attr.name.to_string() == "ry" {
                    if let Some(value) = attr.value.clone() {
                        ry = value.to_string();
                    }
                } else {
                    new_attributes.push(attr);
                }
            }

            if rx == ry ||
                rx == "auto" ||
                ry == "auto" // SVG2
            {
                n.tag_name = "circle".to_string().into();
                let radius = if rx == "auto" { ry } else  { rx };
                new_attributes.push(Attribute {
                    span: DUMMY_SP,
                    namespace: None,
                    prefix: None,
                    name: "r".to_string().into(),
                    raw_name: None,
                    value: Some(radius.into()),
                    raw_value: None,
                });
                n.attributes = new_attributes;
            }
        }
        n.visit_mut_children_with(self);
    }
}

pub fn apply(doc: &mut Document, _: &Params) {
    let mut v = Visitor::new();
    doc.visit_mut_with(&mut v);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::testing::test_plugin;
    use super::*;

    #[testing::fixture("__fixture__/plugins/convertEllipseToCircle.*.svg")]
    fn pass(input: PathBuf) {
        test_plugin(apply, input);
    }
}
