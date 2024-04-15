mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-bindgen-deno-test!");
}

#[wasm_bindgen]
pub fn panic() {
    panic!();
}

#[wasm_bindgen]
pub async fn panic_async() {
    panic!();
}
