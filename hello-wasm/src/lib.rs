// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
use std::fs::File;
use std::io::prelude::*;
use std::{
    fs::{read, read_to_string, write},
    path::Path,
    str::FromStr,
};
use wasm_bindgen::prelude::*;

// The attribute says "wasm-bindgen knows how to find these functions".
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} - I invoked Rust code!", name)
}
