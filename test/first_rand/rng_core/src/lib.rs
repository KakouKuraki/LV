use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn rand_int() -> u32 {
    let num = rand::thread_rng().gen_range(0..=100);
    num
}
