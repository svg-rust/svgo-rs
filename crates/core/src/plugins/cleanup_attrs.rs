// cleanups attributes from newlines, trailing and repeating spaces

use swc_xml_ast::*;
use swc_xml_visit::{VisitMut, VisitMutWith};
use regex::{Regex, Captures};

struct Visitor {
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

pub fn apply(doc: &mut Document) {
    let mut v: Visitor = Default::default();
    doc.visit_mut_with(&mut v);
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use crate::parser::parse_svg;
    use crate::stringifier::{stringify_svg, StringifyOptions};
    use super::*;

    fn code_test(input: String, expected: String) {
        let mut doc = parse_svg(input).unwrap();
        apply(&mut doc);
        let result = stringify_svg(&doc, StringifyOptions {
            pretty: true,
            ..Default::default()
        });
        assert_eq!(result.trim_end(), expected);
    }

    #[test]
    fn test_1() {
        code_test(
            r#"<svg xmlns="  http://www.w3.org/2000/svg
" attr="a      b" attr2="a
b">
    test
</svg>"#.to_string(),
            r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b" attr2="a b">
    test
</svg>"#.to_string(),
        );
    }

    #[test]
    fn test_2() {
        code_test(
            r#"<svg xmlns="  http://www.w3.org/2000/svg
" attr="a      b">
    test &amp; &lt;&amp; &gt; &apos; &quot; &amp;
</svg>"#.to_string(),
            r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b">
    test &amp; &lt;&amp; &gt; &apos; &quot; &amp;
</svg>"#.to_string(),
        );
    }

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
</svg>"#.to_string(),
        r#"<svg xmlns="http://www.w3.org/2000/svg" attr="a b" attr2="a b">
    <foo attr="a b" attr2="a b">
        test
    </foo>
</svg>"#.to_string(),
        );
    }
}
