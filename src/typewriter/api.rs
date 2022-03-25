use wasm_bindgen::prelude::*;
use super::alphabet;
use super::particle;


#[wasm_bindgen]
pub struct Typewriter {
    particles: Vec<u16>,
    size: i16
}

#[wasm_bindgen]
impl Typewriter {
    pub fn new(sx: i16, sy: i16, width: i16, height: i16, text: String) -> Typewriter {
        let (lines, size) = alphabet::text_to_line(
            alphabet::Point{x:sx, y:sy}, width, height, &text
        );

        let points = particle::get_points(lines, size);


        Self {
            particles: points,
            size
        }
    }

    pub fn get_size(&self) -> i16 {
        self.size
    }

    pub fn get_count(&self) -> usize {
        self.particles.len()
    }

    pub fn repellant(&mut self, x: i32, y: i32) {

    }

    pub fn run(&mut self, x: i32, y: i32) {

    }

    pub fn cells(&self) -> *const u16 {
        self.particles.as_ptr()
    }
}