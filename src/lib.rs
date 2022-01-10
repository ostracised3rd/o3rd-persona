mod utils;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(a: i32, b: i32) -> i32 {
    a * b
}


// mod maze_runner;
// mod maze;
// mod runner;
mod maze_runner;