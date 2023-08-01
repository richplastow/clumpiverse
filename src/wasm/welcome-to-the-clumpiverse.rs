use wasm_bindgen::prelude::*;

mod clumpiverse;
mod mass;
mod renderer;

pub use clumpiverse::Clumpiverse;

#[wasm_bindgen]
pub fn greet_wasm() -> String {
  return "Hello, Clumpiverse!".to_string();
}
