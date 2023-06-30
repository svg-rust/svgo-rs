// Round numeric values to the fixed precision,
// remove default 'px' units.

use std::collections::HashMap;

use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use regex::Regex;

// relative to px
fn get_absolute_lengths() -> HashMap<&'static str, f64> {
    HashMap::from([
        ("cm", 96_f64 / 2.54_f64),
        ("mm", 96_f64 / 25.4_f64),
        ("in", 96_f64),
        ("pt", 4_f64 / 3_f64),
        ("pc", 16_f64),
        ("px", 1_f64),
    ])
}

pub fn round(number: f64, precision: i32) -> f64 {
    let scale: f64 = 10_f64.powi(precision);
    (number * scale).round() / scale
  }

fn round_value(value: &str, float_precision: i32) -> String {
    let num = value.parse::<f64>().unwrap_or(0.0);
    if num.is_nan() {
        value.to_string()
    } else {
        format!("{}", round(num, float_precision))
    }
}

fn convert_to_px(value: &str, unit: &str, float_precision: i32) -> String {
    let absolute_lengths = get_absolute_lengths();
    let len = absolute_lengths.get(unit);
    if let Some(len) = len {
        let px_num = round(len * value.parse::<f64>().unwrap_or(0.0), float_precision);
        if px_num.to_string().len() < value.len() {
            px_num.to_string() + "px"
        } else {
            value.to_string()
        }
    } else {
        value.to_string()
    }
}

fn remove_leading_zero(value: &str) -> String {
    if value.starts_with("0") && !value.starts_with("0.") {
        value[1..].to_string()
    } else {
        value.to_string()
    }
}

struct Visitor {
    float_precision: i32,
    leading_zero: bool,
    default_px: bool,
    convert_to_px: bool,
}

impl Default for Visitor {
    fn default() -> Self {
        Self {
            float_precision: 3,
            leading_zero: true,
            default_px: true,
            convert_to_px: true,
        }
    }
}

impl Visitor {
    fn new(params: &Params) -> Self {
        let Params {
            float_precision,
            leading_zero,
            default_px,
            convert_to_px,
        } = *params;

        Self {
            float_precision,
            leading_zero,
            default_px,
            convert_to_px,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_element(&mut self, n: &mut Element) {
        n.visit_mut_children_with(self);

        if let Some(view_box) = n.attributes.iter_mut().find(|attr| attr.name.to_string() == "viewBox") {
            if let Some(value) = view_box.value.clone() {
                let nums: Vec<String> = value.to_string()
                    .split(|c: char| c.is_whitespace() || c == ',')
                    .filter(|s| *s != "")
                    .map(|s| s.to_string())
                    .collect();
                let rounded_nums: Vec<String> = nums
                    .into_iter()
                    .map(|num| round_value(&num, self.float_precision))
                    .collect();
                view_box.value = Some(rounded_nums.join(" ").into());
            }
        }

        let reg_numeric_values = Regex::new(r"^([-+]?\d*\.?\d+([eE][-+]?\d+)?)(px|pt|pc|mm|cm|m|in|ft|em|ex|%)?$").unwrap();
        for attr in n.attributes.iter_mut() {
            // The `version` attribute is a text string and cannot be rounded
            if attr.name.to_string() == "version" {
                continue;
            }

            if let Some(value) = attr.value.clone() {
                if let Some(captures) = reg_numeric_values.captures(&value.to_string()) {
                    let num_str = captures.get(1).map_or("", |m| m.as_str());
                    let unit = captures.get(2).map_or("", |m| m.as_str());

                    let mut v = if self.convert_to_px {
                        convert_to_px(num_str, unit, self.float_precision).into()
                    } else {
                        round_value(num_str, self.float_precision).into()
                    };
                    if self.leading_zero {
                        v = remove_leading_zero(&value).into();
                    }
                    if self.default_px && unit == "px" {
                        v = num_str.into();
                    }

                    attr.value = Some(v);
                }
            }
        }
    }
}

pub struct Params {
    pub float_precision: i32,
    pub leading_zero: bool,
    pub default_px: bool,
    pub convert_to_px: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            float_precision: 3,
            leading_zero: true,
            default_px: true,
            convert_to_px: true,
        }
    }
}

pub fn apply(doc: &mut Document, params: &Params) {
    let mut v: Visitor = Visitor::new(params);
    doc.visit_mut_with(&mut v);
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, borrow::Borrow, path::PathBuf, fs};

    use swc_core::common::{SourceMap, FileName};
    use swc_xml::{
        parser::{parse_file_as_document, parser},
        codegen::{writer::basic::BasicXmlWriter, CodeGenerator, CodegenConfig, Emit},
    };

    #[cfg(test)]
    use pretty_assertions::assert_eq;

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

        apply(&mut doc, &Default::default());

        let mut xml_str = String::new();
        let wr = BasicXmlWriter::new(&mut xml_str, None, Default::default());
        let gen_conf = CodegenConfig {
            minify: true,
            scripting_enabled: false,
            ..Default::default()
        };
        let mut gen = CodeGenerator::new(wr, gen_conf);

        gen.emit(&doc).unwrap();

        assert_eq!(xml_str, expected);
    }

    fn document_test(input: PathBuf) {
        let text = fs::read_to_string(input).unwrap();
        let re = Regex::new(r"\s*@@@\s*").unwrap();
        let fields: Vec<&str> = re.split(&text).collect();

        let input = fields[0].trim();
        let expected = fields[1].trim();

        code_test(input, expected);
    }

    #[testing::fixture("__fixture__/plugins/cleanupNumericValues*.svg")]
    fn pass(input: PathBuf) {
        document_test(input);
    }
}
