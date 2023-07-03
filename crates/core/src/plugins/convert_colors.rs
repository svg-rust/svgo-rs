// Convert [r, g, b] to #rrggbb.

use std::collections::HashMap;

use regex::Regex;
use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use serde::Deserialize;

use crate::collections::{get_colors_props, get_colors_names, get_colors_short_names};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CurrentColor {
    Bool(bool),
    Str(String),
}

/// Convert [r, g, b] to #rrggbb.
fn convert_rgb_to_hex(rgb: &Vec<u8>) -> String {
    let hex_number =
        (u32::from(rgb[0]) << 16) // [r]
        | (u32::from(rgb[1]) << 8) // [r][g]
        | u32::from(rgb[2]); // [r][g][b]
    format!("#{:06X}", hex_number)
}

fn get_short_hex(v: u32) -> u32 {
    ((v & 0x0ff00000) >> 12) | ((v & 0x00000ff0) >> 4)
}

fn get_long_hex(v: u32) -> u32 {
    ((v & 0xf000) << 16)
        | ((v & 0xff00) << 12)
        | ((v & 0x0ff0) << 8)
        | ((v & 0x00ff) << 4)
        | (v & 0x000f)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    #[serde(default = "default_current_color")]
    pub current_color: CurrentColor,
    #[serde(default = "default_names2hex")]
    pub names2hex: bool,
    #[serde(default = "default_rgb2hex")]
    pub rgb2hex: bool,
    #[serde(default = "default_shorthex")]
    pub shorthex: bool,
    #[serde(default = "default_shortname")]
    pub shortname: bool,
}

fn default_current_color() -> CurrentColor {
    CurrentColor::Bool(false)
}

fn default_names2hex() -> bool {
    true
}

fn default_rgb2hex() -> bool {
    true
}

fn default_shorthex() -> bool {
    true
}

fn default_shortname() -> bool {
    true
}

impl Default for Params {
    fn default() -> Self {
        Self {
            current_color: CurrentColor::Bool(false),
            names2hex: true,
            rgb2hex: true,
            shorthex: true,
            shortname: true,
        }
    }
}

struct Visitor<'a> {
    params: &'a Params,

    // Collections
    colors_names: HashMap<&'static str, &'static str>,
    colors_short_names: HashMap<&'static str, &'static str>,
    colors_props: Vec<&'static str>
}

impl<'a> Visitor<'a> {
    fn new(params: &'a Params) -> Self {
        Self {
            params,
            colors_names: get_colors_names(),
            colors_short_names: get_colors_short_names(),
            colors_props: get_colors_props(),
        }
    }
}

impl VisitMut for Visitor<'_> {
    fn visit_mut_attribute(&mut self, n: &mut Attribute) {
        if !self.colors_props.contains(&n.name.to_string().as_str()) {
            return;
        }

        if let Some(value) = &n.value {
            let mut value = value.to_string();

            // convert colors to currentColor
            let matched = match &self.params.current_color {
                CurrentColor::Bool(b) => {
                    if *b {
                        value != "none"
                    } else {
                        false
                    }
                },
                CurrentColor::Str(s) => value == *s,
            };
            if matched {
                value = "currentColor".to_string();
            }

            // convert color name keyword to long hex
            if self.params.names2hex {
                let color_name = value.to_lowercase();
                if let Some(hex) = self.colors_names.get(&color_name.as_str()) {
                    value = hex.to_string();
                }
            }

            // convert rgb() to long hex
            if self.params.rgb2hex {
                let r_number = "([+-]?(?:\\d*\\.\\d+|\\d+\\.?)%?)";
                let r_comma = "\\s*,\\s*";
                let reg_rgb = Regex::new(&format!("^rgb\\(\\s*{}{}{}{}{}", r_number, r_comma, r_number, r_comma, r_number)).unwrap();
                if let Some(caps) = reg_rgb.captures(&value) {
                    let nums: Vec<u8> = caps.iter().skip(1).map(|m| {
                        if let Some(m) = m {
                            let m = m.as_str();
                            let n = if m.ends_with('%') {
                                (m[..m.len() - 1].parse::<f64>().unwrap() * 2.55).round()
                            } else {
                                m.parse::<f64>().unwrap()
                            };
                            n.max(0.0).min(255.0) as u8
                        } else {
                            0
                        }
                    }).collect();
                    if nums.len() == 3 {
                        value = convert_rgb_to_hex(&nums);
                    }
                }
            }

            // convert long hex to short hex
            if self.params.shorthex {
                if value.len() == 7 && value.starts_with('#') {
                    let hex_value = &value[1..];
                    if let Ok(hex) = u32::from_str_radix(hex_value, 16) {
                        let compact = get_short_hex(hex);
                        if hex == get_long_hex(compact) {
                            value = format!("#{:03x}", get_short_hex(hex));
                        }
                    }
                }
            }

            // convert hex to short name
            if self.params.shortname {
                let color_name = value.to_lowercase();
                if let Some(short_name) = self.colors_short_names.get(color_name.as_str()) {
                    value = short_name.to_string();
                }
            }

            n.value = Some(value.into());
        }
    }

    fn visit_mut_element(&mut self, n: &mut Element) {
        n.visit_mut_children_with(self);
    }
}

pub fn apply(doc: &mut Document, params: &Params) {
    let mut v = Visitor::new(params);
    doc.visit_mut_with(&mut v);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::testing::test_plugin;
    use super::*;

    #[testing::fixture("__fixture__/plugins/convertColors*.svg")]
    fn pass(input: PathBuf) {
        test_plugin(apply, input);
    }
}
