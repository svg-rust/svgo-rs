// Collapse useless groups.
//
// # Example
// <g>
//     <g attr1="val1">
//         <path d="..."/>
//     </g>
// </g>
//         ⬇
// <g>
//     <g>
//         <path attr1="val1" d="..."/>
//     </g>
// </g>
//         ⬇
// <path attr1="val1" d="..."/>

use std::collections::{HashSet, HashMap};

use swc_core::common::DUMMY_SP;
use swc_xml_ast::*;
use swc_xml_visit::{VisitMut, VisitMutWith};
use serde::Deserialize;

use crate::collections::{get_elems_groups, get_inheritable_attrs};

#[derive(Debug, Deserialize, Default)]
pub struct Params {
}

struct Visitor {
    elems_groups: HashMap<&'static str, Vec<&'static str>>,
    inheritable_attrs: Vec<&'static str>,
}

impl Default for Visitor {
    fn default() -> Self {
        Self {
            elems_groups: get_elems_groups(),
            inheritable_attrs: get_inheritable_attrs(),
        }
    }
}

impl Visitor {
    fn has_animated_attr(&mut self, n: &Element, name: &str) -> bool {
        if self.elems_groups.get("animation").unwrap().contains(&n.tag_name.to_string().as_str()) &&
            n.attributes.iter().any(|attr| attr.name.to_string() == "attributeName" && attr.value == Some(name.into()))
        {
            return true;
        }
        for child in n.children.iter() {
            if let Child::Element(child) = child {
                if self.has_animated_attr(child, name) {
                    return true;
                }
            }
        }
        false
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, p: &mut Element) {
        p.visit_mut_children_with(self);

        if p.tag_name.to_string() == "switch" {
            return;
        }

        p.children.iter_mut().for_each(|n| {
            if let Child::Element(n) = n {
                // non-empty groups
                if n.tag_name.to_string() != "g" || n.children.len() == 0 {
                    return;
                }

                // move group attibutes to the single child element
                if n.attributes.len() != 0 && n.children.len() == 1 {
                    let mut n_attrs:HashSet::<String> = HashSet::new();
                    n.attributes.iter().for_each(|attr: &Attribute| {
                        n_attrs.insert(attr.name.to_string());
                    });

                    let first_child = &mut n.children[0];
                    // TODO untangle this mess
                    if let Child::Element(first_child) = first_child {
                        let mut first_child_attrs: HashSet::<String> = HashSet::new();
                        first_child
                            .attributes
                            .iter().
                            for_each(|attr: &Attribute| {
                                first_child_attrs.insert(attr.name.to_string());
                            });

                        if !first_child_attrs.contains("id") &&
                            !n_attrs.contains("filter") &&
                            (!n_attrs.contains("class") ||
                                !first_child_attrs.contains("class")) &&
                            ((!n_attrs.contains("clip-path") &&
                                !n_attrs.contains("mask")) ||
                                (first_child.tag_name.to_string() == "g" &&
                                    !n_attrs.contains("transform") &&
                                    !first_child_attrs.contains("transform")))
                        {
                            let mut new_attributes = vec![];
                            for attr in n.attributes.clone().into_iter() {
                                let name = attr.name.to_string();
                                if let Some(value) = attr.value.clone() {
                                    // avoid copying to not conflict with animated attribute
                                    if self.has_animated_attr(&first_child, &name) {
                                        return;
                                    }

                                    let first_child_attr = first_child
                                        .attributes
                                        .iter_mut()
                                        .find(|attr| attr.name.to_string() == name);
                                    match first_child_attr {
                                        None => {
                                            first_child.attributes.push(Attribute {
                                                span: DUMMY_SP,
                                                namespace: None,
                                                prefix: None,
                                                name: name.into(),
                                                raw_name: None,
                                                value: Some(value),
                                                raw_value: None,
                                            });
                                        },
                                        Some(first_child_attr) => {
                                            if name == "transform" {
                                                let new_value = format!("{} {}", value, first_child_attr.value.clone().unwrap_or("".into()).to_string());
                                                first_child_attr.value = Some(new_value.into());
                                            } else if first_child_attr.value == Some("inherit".into()) {
                                                first_child_attr.value = Some(value);
                                            } else if !self.inheritable_attrs.contains(&name.to_string().as_str()) && first_child_attr.value != Some(value) {
                                                return;
                                            }
                                        }
                                    }
                                    continue;
                                }
                                new_attributes.push(attr);
                            }
                            n.attributes = new_attributes;
                        }
                    }
                }
            }
        });

        let mut new_children: Vec<Child> = vec![];
        for child in p.children.iter() {
            if let Child::Element(n) = child {
                if n.tag_name.to_string() != "g" || n.children.len() == 0 {
                    new_children.push(child.clone());
                    continue;
                }

                // collapse groups without attributes
                if n.tag_name.to_string() != "g" || n.attributes.len() == 0 {
                    // animation elements "add" attributes to group
                    // group should be preserved
                    for child in n.children.iter() {
                        if let Child::Element(child) = child {
                            if self.elems_groups.get("animation").unwrap().contains(&child.tag_name.to_string().as_str()) {
                                return;
                            }
                        }
                    }
                    // replace current node with all its children
                    for child in n.children.clone() {
                        new_children.push(child);
                    }
                    continue;
                }
            }
            new_children.push(child.clone());
        }
        p.children = new_children;
    }
}

pub fn apply(doc: &mut Document, _: &Params) {
    let mut v: Visitor = Default::default();
    doc.visit_mut_with(&mut v);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::testing::test_plugin;
    use super::*;

    #[testing::fixture("__fixture__/plugins/collapseGroups.*.svg")]
    fn pass(input: PathBuf) {
        test_plugin(apply, input);
    }
}
