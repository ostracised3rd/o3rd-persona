use wasm_bindgen::prelude::*;
use crate::utils::vector::{Line, Vector};
use super::typewriter::{lettering};
use super::particle::{Particle};


#[wasm_bindgen]
pub struct Words {
    cells: Vec<i32>,
    particles: Vec<Particle>,
    size: i32
}


#[wasm_bindgen]
impl Words {
    pub fn new(sx: i32, sy: i32, width: i32, height: i32, text: String) -> Words {
        let (lines, size) = lettering(Vector{x:sx, y:sy}, Vector{x:width, y:height}, &text);
        let size = size / 5;
        let (cells, particles) = rest_in_pieces(lines, size);

        Self {
            cells,
            particles,
            size
        }
    }

    pub fn get_particle_size(&self) -> i32 {
        self.size
    }

    pub fn get_cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn cells(&self) -> *const i32 {
        self.cells.as_ptr()
    }

    pub fn repellant(&mut self, _x: i32, _y: i32) {

    }

    pub fn run(&mut self, _x: i32, _y: i32) {
        for p in self.particles.iter_mut() {
            let (i, pos) = p.run();
            self.cells[i] = pos.x;
            self.cells[i+1] = pos.y;
        }
    }
}


pub fn disintegrate(line: Line, size: f64, index: usize) -> (Vec<i32>, Vec<Particle>) {

    let mut px = line.start.x as f64;
    let mut py = line.start.y as f64;
    let lx = (line.end.x - line.start.x) as f64;
    let ly = (line.end.y - line.start.y) as f64;
    let mag = (lx.powf(2.0) + ly.powf(2.0)).sqrt();
    let vx = (lx / mag) * size;
    let vy = (ly / mag) * size;
    let count = (mag / size) as usize;

    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();
    for i in 0..count {
        cells.push(px as i32);
        cells.push(py as i32);

        particles.push(Particle::new(
            index+(i*2), 
            size as i32,
            Vector {x: px as i32, y: py as i32}, 
        ));

        px += vx;
        py += vy;
    }

    (cells, particles)
}

pub fn rest_in_pieces(lines: Vec<Line>, size: i32) -> (Vec<i32>, Vec<Particle>) {
    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();
    let size = size as f64;

    for line in lines {
        let (lc, lp) = disintegrate(line, size, cells.len());
        cells.extend(lc);
        particles.extend(lp);
    }

    (cells, particles)
}
