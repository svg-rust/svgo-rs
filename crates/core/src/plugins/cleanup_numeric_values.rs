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

/// Remove floating-point numbers leading zero.
/// 
/// # Examples
/// 0.5 → .5
///
/// -0.5 → -.5
fn remove_leading_zero(num: f64) -> String {
    let mut str_num = num.to_string();

    if 0.0 < num && num < 1.0 && str_num.chars().next() == Some('0') {
        str_num = str_num[1..].to_string();
    } else if -1.0 < num && num < 0.0 && str_num.chars().nth(1) == Some('0') {
        str_num = str_num.chars().take(1).chain(str_num.chars().skip(2)).collect();
    }

    str_num
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

        let reg_numeric_values = Regex::new(r#"^([-+]?\d*\.?\d+([eE][-+]?\d+)?)(px|pt|pc|mm|cm|m|in|ft|em|ex|%)?$"#).unwrap();
        for attr in n.attributes.iter_mut() {
            // The `version` attribute is a text string and cannot be rounded
            if attr.name.to_string() == "version" {
                continue;
            }

            if let Some(value) = attr.value.clone() {
                if let Some(captures) = reg_numeric_values.captures(&value.to_string()) {
                    let num_str = captures.get(1).map_or("", |m| m.as_str());
                    let unit = captures.get(3).map_or("", |m| m.as_str());

                    // round it to the fixed precision
                    let mut num = round(num_str.parse::<f64>().unwrap_or(0.0), self.float_precision);
                    let mut units = unit;

                    // convert absolute values to pixels
                    if self.convert_to_px {
                        let absolute_lengths = get_absolute_lengths();
                        let len = absolute_lengths.get(unit);
                        if let Some(len) = len {
                            let px_num = round(len * num_str.parse::<f64>().unwrap_or(0.0), self.float_precision);
                            if px_num.to_string().len() < value.len() {
                                num = px_num;
                                units = "px";
                            }
                        }
                    }

                    // and remove leading zero
                    let str = if self.leading_zero {
                        remove_leading_zero(num)
                    } else {
                        num.to_string()
                    };

                    // remove default 'px' units
                    if self.default_px && units == "px" {
                        units = "";
                    }

                    attr.value = Some(format!("{}{}", str, units).into());
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
    use std::path::PathBuf;

    use crate::testing::test_plugin;
    use super::*;

    #[testing::fixture("__fixture__/plugins/cleanupNumericValues*.svg")]
    fn pass(input: PathBuf) {
        test_plugin(
            |doc| apply(doc, &Default::default()),
            input,
        );
    }
}
