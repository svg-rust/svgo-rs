#![deny(clippy::all)]

#[cfg(feature = "node")]
#[macro_use]
extern crate napi_derive;

mod collections;
mod parser;
mod plugins;
mod stringifier;

#[cfg(test)]
mod testing;

use stringifier::{stringify_svg, StringifyOptions};

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
    let mut doc = parser::parse_svg(input).unwrap();

    plugins::cleanup_attrs::apply(&mut doc);
    plugins::cleanup_enable_background::apply(&mut doc);
    plugins::cleanup_ids::apply(&mut doc, &Default::default());
    plugins::cleanup_numeric_values::apply(&mut doc, &Default::default());

    let data = stringify_svg(&doc, StringifyOptions {
        pretty: true,
        ..Default::default()
    });
    Output {
        data,
    }
}
