use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn explain(input: &str) -> String {
    // reverse the string
    input.chars().rev().collect()
}