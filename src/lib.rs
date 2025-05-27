mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str, count: u32) {
    alert(format!("Hello, {}! You have {} money :O", name, count).as_str());
}
