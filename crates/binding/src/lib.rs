#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use svgo_rs::{optimize as optimize_core, Output};

/// The core of SVGO RS
#[napi]
pub fn optimize(input: String) -> Output {
    optimize_core(input)
}
