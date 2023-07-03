use std::sync::Arc;

use regex::Regex;
use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
    parser::{parse_file_as_document, parser, error::Error},
};
use swc_core::common::{SourceMap, FileName};

use crate::collections::get_text_elems;

struct Visitor {
    text_elems: Vec<&'static str>,
}

impl VisitMut for Visitor {
    fn visit_mut_comment(&mut self, n: &mut Comment) {
        n.data = n.data.to_string().trim().into();
    }

    fn visit_mut_element(&mut self, n: &mut Element) {
        let mut children: Vec<Child> = vec![];
        n.children.iter_mut().for_each(|child| {
            if let Child::Text(text) = child {
                if self.text_elems.contains(&n.tag_name.to_string().as_str()) {
                    children.push(child.clone())
                } else {
                    let re = Regex::new(r"\S").unwrap();
                    if re.is_match(&text.data.to_string()) {
                        text.data = text.data.to_string().trim().into();
                        children.push(child.clone())
                    }
                }
            } else {
                children.push(child.clone())
            }
        });
        n.children = children;

        n.visit_mut_children_with(self)
    }
}

impl Visitor {
    fn new() -> Self {
        Self {
            text_elems: get_text_elems(),
        }
    }
}

pub fn parse_svg(input: String) -> Result<Document, Error> {
    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, input);

    let mut errors = vec![];
    let mut r = parse_file_as_document(
        &fm,
        parser::ParserConfig::default(),
        &mut errors
    );

    match &mut r {
        Ok(doc) => {
            let mut v = Visitor::new();
            doc.visit_mut_with(&mut v);
            r
        },
        Err(_) => r
    }
}
