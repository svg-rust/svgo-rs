use std::{sync::Arc, borrow::Borrow, path::PathBuf, fs};

use regex::Regex;
use serde::de::DeserializeOwned;
use swc_core::common::{SourceMap, FileName};
use swc_xml::{
    ast::Document,
    parser::{parse_file_as_document, parser},
    codegen::{writer::basic::BasicXmlWriter, CodeGenerator, CodegenConfig, Emit},
};

#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn test_plugin<F, P>(
    apply: F,
    input: PathBuf,
) where
    F: FnOnce(&mut Document, &P),
    P: DeserializeOwned + Default
{
    let text = fs::read_to_string(input).unwrap();
    let re = Regex::new(r"\s*@@@\s*").unwrap();
    let fields: Vec<&str> = re.split(&text).collect();

    let input = fields[0].trim();
    let expected = fields[1].trim();
    let params: P = if fields.len() > 2 {
        let json_str = fields[2].trim();
        serde_json::from_str(&json_str).unwrap()
    } else {
        Default::default()
    };

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