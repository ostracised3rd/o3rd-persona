use wasm_bindgen::prelude::*;
use crate::utils::maths::{Line, Vector, sub_vectors, add_vectors};
use super::typewriter::{lettering};
use super::particle::{Particle};


#[wasm_bindgen]
pub struct Words {
    // lines: Vec<i32>,
    cells: Vec<i32>,
    particles: Vec<Particle>,
    size: f64
}


#[wasm_bindgen]
impl Words {
    pub fn new(sx: i32, sy: i32, width: i32, height: i32, text: String) -> Words {
        let screen = Vector{x:width as f64, y:height as f64};
        let (lines, size) = lettering(
            Vector{x:sx as f64, y:sy as f64}, 
            screen.clone(), 
            &text
        );

        // let mut l = Vec::<i32>::new();

        // for line in lines.iter() {
        //     l.push(line.start.x as i32);
        //     l.push(line.start.y as i32);
        //     l.push(line.end.x as i32);
        //     l.push(line.end.y as i32);
        // }

        let size = size / 5.;
        let (cells, particles) = rest_in_pieces(screen, lines, size);

        Self {
            // lines: l,
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

    // pub fn get_line_count(&self) -> usize {
    //     self.lines.len()
    // }

    // pub fn lining(&self) -> *const i32 {
    //     self.lines.as_ptr()
    // }

    // pub fn repellent(&mut self, x: i32, y: i32) {
    //     let point = Vector{x: x as f64, y: y as f64};
    //     for p in self.particles.iter_mut() {
    //         p.repellent(point);
    //     }
    // }

    pub fn run(&mut self, x: i32, y: i32, boom: bool, gx: i32, gy: i32) {
        let gravity = Vector{x: x as f64, y: y as f64};
        let grenade = Vector{x: gx as f64, y: gy as f64};
        for p in self.particles.iter_mut() {
            let (i, pos) = p.run(gravity, boom, grenade);
            self.cells[i] = pos.x as i32;
            self.cells[i+1] = pos.y as i32;
        }
    }
}


pub fn disintegrate(screen: Vector, line: Line, size: f64, index: usize) -> (Vec<i32>, Vec<Particle>) {

    
    
    // let lx = line.end.x - line.start.x;
    // let ly = line.end.y - line.start.y;
    // let mag = (lx.powf(2.) + ly.powf(2.)).sqrt();
    // let vx = (lx / mag) * size;
    // let vy = (ly / mag) * size;
    

    let mut dist = sub_vectors(line.end, line.start);
    let mag = dist.mag();
    dist.set_mag(size);
    let count = (mag / size) as usize;

    let mut spacing = dist.clone();
    let missing = (mag - size * count as f64) / (count-1) as f64;
    spacing.set_mag(missing);


    let mut px = line.start.x;
    let mut py = line.start.y;
    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();



    for i in 0..count {
        cells.push(px as i32);
        cells.push(py as i32);

        particles.push(Particle::new(
  
            index+(i*2), 
            screen,
            size,
            Vector {x: px, y: py}, 
        ));

        px += dist.x + spacing.x;
        py += dist.y + spacing.y;
    }

    (cells, particles)
}


pub fn rest_in_pieces(screen: Vector, lines: Vec<Line>, size: f64) -> (Vec<i32>, Vec<Particle>) {
    let mut cells = Vec::<i32>::new();
    let mut particles = Vec::<Particle>::new();

    for line in lines {
        let (lc, lp) = disintegrate(screen, line, size, cells.len());
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

        // words.run(200, 200);
        // println!("{:?}", &words.cells);
    }
}