#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{sync::Arc, borrow::Borrow};

use swc_xml::{parser::{parse_file_as_document, parser}};
use swc_core::common::{SourceMap, FileName};

mod plugins;

#[napi(object)]
pub struct Output {
    pub data: String,
}

#[napi(object)]
pub struct Config {
    pub path: Option<String>,
}

#[napi]
pub fn optimize(input: String, config: Option<Config>) -> Output {
    let cm = Arc::<SourceMap>::default();
    let fm = cm.new_source_file(FileName::Anon, input);

    let mut errors = vec![];
    let document = parse_file_as_document(
        fm.borrow(),
        parser::ParserConfig::default(),
        &mut errors
    ).unwrap();

    todo!()
}
