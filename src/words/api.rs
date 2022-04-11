use wasm_bindgen::prelude::*;
use crate::utils::maths::{Line, Vector};
use super::typewriter::{lettering};
use super::particle::{Particle};


#[wasm_bindgen]
pub struct Words {
    cells: Vec<i32>,
    particles: Vec<Particle>,
    size: f64
}


#[wasm_bindgen]
impl Words {
    pub fn new(sx: i32, sy: i32, width: i32, height: i32, text: String) -> Words {
        let (lines, size) = lettering(
            Vector{x:sx as f64, y:sy as f64}, 
            Vector{x:width as f64, y:height as f64}, 
            &text
        );

        let size = size / 5.;
        let (cells, particles) = rest_in_pieces(width as f64, height as f64, lines, size);

        Self {
            cells,
            particles,
            size
        }
    }

    pub fn get_particle_size(&self) -> i32 {
        self.size as i32
    }

    pub fn get_cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn cells(&self) -> *const i32 {
        self.cells.as_ptr()
    }

    pub fn repellent(&mut self, x: i32, y: i32) {
        let point = Vector{x: x as f64, y: y as f64};
        for p in self.particles.iter_mut() {
            p.repellent(point);
        }
    }

    pub fn run(&mut self, x: i32, y: i32) {
        let point = Vector{x: x as f64, y: y as f64};
        for p in self.particles.iter_mut() {
            let (i, pos) = p.run(point);
            self.cells[i] = pos.x as i32;
            self.cells[i+1] = pos.y as i32;
        }
    }
}


pub fn disintegrate(width: f64, height: f64, line: Line, size: f64, index: usize) -> (Vec<i32>, Vec<Particle>) {

    let mut px = line.start.x;
    let mut py = line.start.y;
    let lx = line.end.x - line.start.x;
    let ly = line.end.y - line.start.y;
    let mag = (lx.powf(2.) + ly.powf(2.)).sqrt();
    let vx = (lx / mag) * size;
    let vy = (ly / mag) * size;
    let count = (mag / size) as usize;

    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();
    for i in 0..count {
        cells.push(px as i32);
        cells.push(py as i32);

        particles.push(Particle::new(
            width, 
            height,
            index+(i*2), 
            size,
            Vector {x: px, y: py}, 
        ));

        px += vx;
        py += vy;
    }

    (cells, particles)
}


pub fn rest_in_pieces(width: f64, height: f64, lines: Vec<Line>, size: f64) -> (Vec<i32>, Vec<Particle>) {
    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();

    for line in lines {
        let (lc, lp) = disintegrate(width, height, line, size, cells.len());
        cells.extend(lc);
        particles.extend(lp);
    }

    (cells, particles)
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn overlapping() {
        let text = "SOHEIL\n DEVELOPER AND\n SOME OTHER THINGS";

        let mut words = Words::new(0,0, 500, 500, text.to_string());

        words.run(200, 200);
        println!("{:?}", &words.cells);
    }
}