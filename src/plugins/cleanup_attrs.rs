// cleanups attributes from newlines, trailing and repeating spaces

use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use regex::{Regex, Captures};

pub struct Visitor {
    newlines: bool,
    trim: bool,
    spaces: bool,
}

impl Default for Visitor {
    fn default() -> Self {
        Self {
            newlines: true,
            trim: true,
            spaces: true,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, n: &mut Element) {
        n.visit_mut_children_with(self);

        for attr in n.attributes.iter_mut() {
            if attr.value.is_none() {
                break;
            }

            let mut value = attr.value.clone().unwrap().to_string();

            if self.newlines {
                // new line which requires a space instead of themselve
                let reg_newlines_need_space = Regex::new(r#"(\S)\r?\n(\S)"#).unwrap();
                value = reg_newlines_need_space.replace_all(&value, |caps: &Captures| format!("{} {}", &caps[1], &caps[2])).to_string();

                // simple new line
                let reg_new_lines = Regex::new(r#"\r?\n"#).unwrap();
                value = reg_new_lines.replace_all(&value, |_: &Captures| "").to_string()
            }

            if self.trim {
                value = value.trim().to_string();
            }

            if self.spaces {
                let reg_spaces = Regex::new(r#"\s{2,}"#).unwrap();
                value = reg_spaces.replace_all(&value, |_: &Captures| " ").to_string()
            }

            attr.value = Some(value.into());
        }
    }
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
        let mut document = parse_file_as_document(
            fm.borrow(),
            parser::ParserConfig::default(),
            &mut errors
        ).unwrap();

        let mut v: Visitor = Default::default();
        document.visit_mut_with(&mut v);

        let mut xml_str = String::new();
        let wr = BasicXmlWriter::new(&mut xml_str, None, BasicXmlWriterConfig::default());
        let mut gen = CodeGenerator::new(wr, CodegenConfig::default());

        gen.emit(&document).unwrap();

        assert_eq!(xml_str, expected);
    }

    #[test]
    fn test_1() {
        code_test(
            r#"<svg xmlns="  http://www.w3.org/2000/svg
" attr="a      b" attr2="a
b">
    test
</svg>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b" attr2="a b">
    test
</svg>"#,
        );
    }

// TODO: swc xml parser has bug for print excaped charactor.

//     #[test]
//     fn test_2() {
//         code_test(
//             r#"<svg xmlns="  http://www.w3.org/2000/svg
// " attr="a      b">
//     test &amp; &lt;&amp; &gt; &apos; &quot; &amp;
// </svg>"#,
//             r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b">
//     test &amp; &lt;&amp; &gt; &apos; &quot; &amp;
// </svg>"#,
//         );
//     }

    #[test]
    fn test_3() {
        code_test(
            r#"<svg xmlns="  http://www.w3.org/2000/svg
" attr="a      b" attr2="a
b">
    <foo attr="a      b" attr2="a
    b">
        test
    </foo>
</svg>"#,
        r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b" attr2="a b">
    <foo attr="a b" attr2="a b">
        test
    </foo>
</svg>"#,
        );
    }
}
