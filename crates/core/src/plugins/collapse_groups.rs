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

use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

struct Visitor;

impl Default for Visitor {
    fn default() -> Self {
        Self
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, n: &mut Element) {
        n.visit_mut_children_with(self);
    }
}

pub fn apply(doc: &mut Document) {
    let mut v: Visitor = Default::default();
    doc.visit_mut_with(&mut v);
}

// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use crate::testing::test_plugin;
//     use super::*;

//     #[testing::fixture("__fixture__/plugins/collapseGroups*.svg")]
//     fn pass(input: PathBuf) {
//         test_plugin(
//             |doc| apply(doc),
//             input,
//         );
//     }
// }
