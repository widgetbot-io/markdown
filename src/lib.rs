mod utils;
mod tokenizer;
mod parser;

use wasm_bindgen::prelude::*;
use crate::parser::parser;
use crate::tokenizer::tokenize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse_markdown(input: String) -> Result<JsValue, JsValue> {
    let tokens = tokenize(&mut input.chars().peekable());
    let ast = parser(&mut tokens.iter().peekable());

    Ok(serde_wasm_bindgen::to_value(&ast)?)
}
