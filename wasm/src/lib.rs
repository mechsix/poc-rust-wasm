mod utils;
mod hello;
mod pi;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn alert_hello() {
    alert(hello::hello())
}

#[wasm_bindgen]
pub fn gen_hello() -> String {
    format!("Hello 2")
}

#[wasm_bindgen]
pub fn pi_by_thread() -> f64 {
    // Thread is unreachable in wasm
    pi::calc_pi_thread()
}

#[wasm_bindgen]
pub fn pi_by_loop() -> f64  {
    pi::calc_pi_loop()
}
