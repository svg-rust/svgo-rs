/// remove or cleanup enable-background attribute when possible
///
/// @see https://www.w3.org/TR/SVG11/filters.html#EnableBackgroundProperty
/// 
/// @example
/// <svg width="100" height="50" enable-background="new 0 0 100 50">
///             ⬇
/// <svg width="100" height="50">

use swc_xml::{
    ast::*,
    visit::{VisitMut, Visit, VisitWith, VisitMutWith},
};
use regex::Regex;

struct Visitor {
    has_filter: bool,
}

impl Visitor {
    pub fn new(has_filter: bool) -> Visitor {
        Self {
            has_filter,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, n: &mut Element) {
        n.visit_mut_children_with(self);

        let enable_background_index = n.attributes.iter().position(|attr| attr.name.to_string() == "enable-background");
        if enable_background_index.is_none() {
            return;
        }

        let enable_background_index = enable_background_index.unwrap();

        if self.has_filter {
            let tag_name = n.tag_name.to_string();

            let height_index = n.attributes.iter().position(|attr| attr.name.to_string() == "height" && attr.value.is_some());
            let width_index = n.attributes.iter().position(|attr| attr.name.to_string() == "width" && attr.value.is_some());

            if (tag_name == "svg" || tag_name == "mask" || tag_name == "pattern") && height_index.is_some() && width_index.is_some() {
                let value = match n.attributes[enable_background_index].value {
                    Some(ref value) => value.to_string(),
                    None => "".to_string(),
                };
                let height = n.attributes[height_index.unwrap()].value.clone().unwrap().to_string();
                let width = n.attributes[width_index.unwrap()].value.clone().unwrap().to_string();

                let reg_enable_background = Regex::new(r#"^new\s0\s0\s([-+]?\d*\.?\d+([eE][-+]?\d+)?)\s([-+]?\d*\.?\d+([eE][-+]?\d+)?)$"#).unwrap();
                let captures = reg_enable_background.captures(&value);

                if captures.is_some() {
                    let captures = captures.unwrap();

                    if captures[1] == width && captures[3] == height {
                        if tag_name == "svg" {
                            n.attributes.remove(enable_background_index);
                        } else {
                            n.attributes[enable_background_index].value = Some("new".into());
                        }
                    }
                }
            }   
        } else {
            // we don't need 'enable-background' if we have no filters
            n.attributes.remove(enable_background_index);
        }
    }
}

pub struct FilterVisitor {
    has_filter: bool,
}

impl Default for FilterVisitor {
    fn default() -> Self {
        Self {
            has_filter: false,
        }
    }
}

impl Visit for FilterVisitor {
    fn visit_element(&mut self, n: &Element) {
        if n.tag_name.to_string() == "filter" {
            self.has_filter = true
        } else {
            n.visit_children_with(self);
        }
    }
}

pub fn apply(doc: &Document) -> Box<dyn VisitMut> {
    let mut filter_visitor: FilterVisitor = Default::default();
    doc.visit_with(&mut filter_visitor);

    let v = Visitor::new(filter_visitor.has_filter);

    Box::new(v)
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, borrow::Borrow};

    use swc_core::common::{SourceMap, FileName};
    use swc_xml::{
        parser::{parse_file_as_document, parser},
        codegen::{
            writer::basic::{BasicXmlWriter, BasicXmlWriterConfig},
            CodeGenerator, CodegenConfig, Emit,
        },
        visit::VisitMutWith,
    };

    use super::*;

    fn code_test(input: &str, expected: &str) {
        let cm = Arc::<SourceMap>::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        let mut errors = vec![];
        let mut doc = parse_file_as_document(
            fm.borrow(),
            parser::ParserConfig::default(),
            &mut errors
        ).unwrap();

        let mut v = apply(&doc);
        doc.visit_mut_with(&mut v);

        let mut xml_str = String::new();
        let wr = BasicXmlWriter::new(&mut xml_str, None, BasicXmlWriterConfig::default());
        let mut gen = CodeGenerator::new(wr, CodegenConfig::default());

        gen.emit(&doc).unwrap();

        assert_eq!(xml_str, expected);
    }

    #[test]
    fn test_1() {
        code_test(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100.5" height=".5" enable-background="new 0 0 100.5 .5">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    test
</svg>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="100.5" height=".5">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    test
</svg>"#,
        );
    }

    #[test]
    fn test_2() {
        code_test(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" enable-background="new 0 0 100 50">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    test
</svg>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" enable-background="new 0 0 100 50">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    test
</svg>"#,
        );
    }

    #[test]
    fn test_3() {
        code_test(
            r#"<svg xmlns="http://www.w3.org/2000/svg">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    <mask width="100" height="50" enable-background="new 0 0 100 50">
        test
    </mask>
</svg>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg">
    <defs>
        <filter id="ShiftBGAndBlur">
            <feOffset dx="0" dy="75" />
        </filter>
    </defs>
    <mask width="100" height="50" enable-background="new">
        test
    </mask>
</svg>"#,
        );
    }

    #[test]
    fn test_4() {
        code_test(
            r#"<svg xmlns="http://www.w3.org/2000/svg">
    <mask width="100" height="50" enable-background="new 0 0 100 50">
        test
    </mask>
</svg>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg">
    <mask width="100" height="50">
        test
    </mask>
</svg>"#,
        );
    }
}
