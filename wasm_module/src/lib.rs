
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(input: &str) -> String {
    format!("draw as: {}", input)
}