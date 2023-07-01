#![deny(clippy::all)]

#[cfg(feature = "node")]
#[macro_use]
extern crate napi_derive;

use std::sync::Arc;

use swc_xml::{
    parser::{parse_file_as_document, parser},
    codegen::{writer::basic::BasicXmlWriter, CodeGenerator, CodegenConfig, Emit},
};
use swc_core::common::{SourceMap, FileName};

mod plugins;

#[cfg(feature = "node")]
#[napi(object)]
pub struct Output {
    pub data: String,
}

#[cfg(not(feature = "node"))]
pub struct Output {
    pub data: String,
}

/// The core of SVGO
pub fn optimize(input: String) -> Output {
    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, input);

    let mut errors = vec![];
    let mut doc = parse_file_as_document(
        &fm,
        parser::ParserConfig::default(),
        &mut errors
    ).unwrap();

    plugins::cleanup_attrs::apply(&mut doc);
    plugins::cleanup_enable_background::apply(&mut doc);
    plugins::cleanup_ids::apply(&mut doc, &Default::default());
    plugins::cleanup_numeric_values::apply(&mut doc, &Default::default());

    let mut xml_str = String::new();
    let wr = BasicXmlWriter::new(&mut xml_str, None, Default::default());
    let gen_conf = CodegenConfig {
        minify: true,
        scripting_enabled: false,
        ..Default::default()
    };
    let mut gen = CodeGenerator::new(wr, gen_conf);
    gen.emit(&doc).unwrap();

    Output {
        data: xml_str
    }
}
