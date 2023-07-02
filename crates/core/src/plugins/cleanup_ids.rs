// Remove unused and minify used IDs
// (only if there are no any <style> or <script>).

use std::collections::{HashMap, HashSet};

use swc_xml::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use regex::Regex;
use linked_hash_map::LinkedHashMap;
use serde::Deserialize;

use super::collections;

struct EnterVisitor<'a> {
    // Options
    force: bool,

    // States
    references_props: Vec<&'static str>,
    deoptimized: bool,
    node_by_id: LinkedHashMap<String, &'a mut Element>,
    references_by_id: HashMap<String, Vec<(&'a mut Element, String, String)>>,
}

impl EnterVisitor<'_> {
    fn new(params: &Params) -> Self {
        let Params {
            force,
            ..
        } = *params;

        Self {
            force,

            references_props: collections::get_references_props(),
            deoptimized: false,
            node_by_id: LinkedHashMap::new(),
            references_by_id: HashMap::new(),
        }
    }
}

impl VisitMut for EnterVisitor<'_> {
    fn visit_mut_element(&mut self, n: &mut Element) {
        let tag_name = n.tag_name.to_string();

        if self.force == false {
            if (tag_name == "style" ||tag_name == "script") && n.children.len() != 0 {
                self.deoptimized = true;

                n.visit_mut_children_with(self);        
                return
            }

            // avoid removing IDs if the whole SVG consists only of defs
            if tag_name == "svg" {
                let has_defs_only = !n.children.iter().any(|child| {
                    match child {
                        Child::Element(child) => child.tag_name.to_string() != "defs",
                        Child::Text(child) => child.data.to_string().trim() != "",
                        _ => true,
                    }
                });
                if has_defs_only {
                    return
                }
            }
        }

        let attributes = n.attributes.clone();
        for (index, attr) in attributes.into_iter().enumerate() {
            let name = attr.name.to_string();
            let value = match &attr.value {
                Some(value) => value,
                None => "",
            };

            if name == "id" {
                // collect all ids
                if self.node_by_id.contains_key(value) {
                    n.attributes.remove(index); // remove repeated id
                } else {
                    unsafe {
                        let r = n as *mut Element;
                        self.node_by_id.insert(value.to_string(), r.as_mut().unwrap());
                    }
                }
            } else {
                // collect all references
                let id = if self.references_props.contains(&name.as_str()) {
                    let reg_references_url = Regex::new(r#"\burl\((["'])?#(.+?)(["'])?\)"#).unwrap();
                    let captures = reg_references_url.captures(&value);
                    match captures {
                        Some(captures) => Some(captures.get(2).unwrap().as_str()), // url() reference
                        None => None,
                    }
                } else if name == "href" || name.ends_with(":href") {
                    let reg_references_href = Regex::new(r#"^#(.+?)$"#).unwrap();
                    let captures = reg_references_href.captures(&value);
                    match captures {
                        Some(captures) => Some(captures.get(1).unwrap().as_str()), // href reference
                        None => None,
                    }
                } else if name == "begin" {
                    let reg_references_begin = Regex::new(r#"(\D+)\."#).unwrap();
                    let captures = reg_references_begin.captures(&value);
                    match captures {
                        Some(captures) => Some(captures.get(1).unwrap().as_str()), // href reference
                        None => None,
                    }
                } else {
                    None
                };

                if let Some(id) = id {
                    let refs = self.references_by_id.get_mut(id);
                    match refs {
                        Some(refs) => {
                            unsafe {
                                let r = n as *mut Element;
                                refs.push((r.as_mut().unwrap(), name, value.to_string()));
                            }
                        },
                        None => {
                            let refs = unsafe {
                                let r = n as *mut Element;
                                vec![(r.as_mut().unwrap(), name, value.to_string())]
                            };
                            self.references_by_id.insert(id.to_string(), refs);
                        }
                    }
                }
            }
        }

        n.visit_mut_children_with(self);
    }
}

// Check if an ID starts with any one of a list of strings.
fn has_string_prefix(str: &String, prefixes: &Vec<String>) -> bool {
    for prefix in prefixes {
        if str.starts_with(prefix.as_str()) {
            return true
        }
    }
    false
}

fn get_generate_id_chars() -> Vec<&'static str> {
    vec![
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
    ]
}

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(default = "default_remove")]
    pub remove: bool,
    #[serde(default = "default_minify")]
    pub minify: bool,
    #[serde(default)]
    pub preserve: Vec<String>,
    #[serde(default, rename = "preservePrefixes")]
    pub preserve_prefixes: Vec<String>,
    #[serde(default)]
    pub force: bool,
}

fn default_remove() -> bool {
    true
}

fn default_minify() -> bool {
    true
}

impl Default for Params {
    fn default() -> Self {
        Params {
            remove: true,
            minify: true,
            preserve: vec![],
            preserve_prefixes: vec![],
            force: false,
        }
    }
}

pub fn apply(doc: &mut Document, params: &Params) {
    let Params {
        remove,
        minify,
        preserve,
        preserve_prefixes,
        ..
    } = params;

    let mut v = EnterVisitor::new(params);
    doc.visit_mut_with(&mut v);

    if v.deoptimized {
        return;
    }

    let preserve_ids: HashSet<String> = preserve.iter().map(|x| x.clone()).collect();

    let is_id_preserved = |id: &String| preserve_ids.get(id).is_some() || has_string_prefix(id, preserve_prefixes);

    let generate_id_chars = get_generate_id_chars();
    let max_id_index: usize = generate_id_chars.len() - 1;

    // Generate unique minimal ID.
    let generate_id = |current_id: &mut Vec<usize>| {
        let len = current_id.len();
        if len > 0 {
            current_id[len - 1] += 1;
            for i in (1..len).rev() {
                if current_id[i] > max_id_index {
                    current_id[i] = 0;
                    if let Some(v) = current_id.get_mut(i - 1) {
                        *v += 1;
                    }
                }
            }
            if current_id[0] > max_id_index {
                current_id[0] = 0;
                current_id.insert(0, 0);
            }
        } else {
            current_id.push(0)
        }
    };

    // Get string from generated ID array.
    let get_id_string = |arr: &Vec<usize>| -> String {
        arr.iter()
            .map(|&i| generate_id_chars[i].to_string())
            .collect::<String>()
    };

    let mut non_referenced_ids: Vec<(&String, &mut Element)> = vec![];
    let mut current_id = vec![];
    for (id, n) in v.node_by_id.iter_mut() {
        let refs = v.references_by_id.get_mut(id);
        match refs {
            Some(refs) => {
                // replace referenced IDs with the minified ones
                if *minify && !is_id_preserved(id) {
                    generate_id(&mut current_id);
                    let mut current_id_string: String = get_id_string(&current_id);
                    while is_id_preserved(&current_id_string) {
                        generate_id(&mut current_id);
                        current_id_string = get_id_string(&current_id);
                    }
                    let index = n.attributes.iter().position(|attr| attr.name.to_string() == "id");
                    if let Some(index) = index {
                        n.attributes[index].value = Some(current_id_string.clone().into());
                    }
                    for (element, name, value) in refs {
                        if value.contains('#') {
                            // replace id in href and url()
                            let attr = element.attributes.iter_mut().find(|attr| attr.name.to_string() == *name);
                            if let Some(attr) = attr {
                                attr.value = Some(value.replace(&format!("#{}", id), &format!("#{}", current_id_string)).into());
                            }
                        } else {
                            // replace id in begin attribute
                            let attr = element.attributes.iter_mut().find(|attr| attr.name.to_string() == *name);
                            if let Some(attr) = attr {
                                attr.value = Some(value.replace(&format!("{}.", id), &format!("{}.", current_id_string)).into());
                            }
                        }
                    }
                }
            },
            None => {
                non_referenced_ids.push((id, n));
            }
        }
    }

    // remove non-referenced IDs attributes from elements
    if *remove {
        for (id, n) in non_referenced_ids {
            if !is_id_preserved(&id) {
                let index = n.attributes.iter().position(|attr| attr.name.to_string() == "id");
                if let Some(index) = index {
                    n.attributes.remove(index);
                }
            }
        }
    }
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

    fn code_test(input: &str, expected: &str, params: &Params) {
        let cm = Arc::<SourceMap>::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        let mut errors = vec![];
        let mut doc = parse_file_as_document(
            fm.borrow(),
            parser::ParserConfig::default(),
            &mut errors
        ).unwrap();

        apply(&mut doc, &params);

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
        let params: Params = if fields.len() > 2 {
            let json_str = fields[2].trim();
            serde_json::from_str(json_str).unwrap()
        } else {
            Default::default()
        };

        code_test(input, expected, &params);
    }

    #[testing::fixture("__fixture__/plugins/cleanupIds*.svg")]
    fn pass(input: PathBuf) {
        document_test(input);
    }
}
