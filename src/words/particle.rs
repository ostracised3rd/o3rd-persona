// use super::typewriter::{Line, Vector};

use crate::utils::maths::{Vector, mapping, sub_vectors};

const MAX_SPEED: f64 = 10.;
const MAX_FORCE: f64 = 1.;


pub struct Particle {
    index: usize,
    pos: Vector,
    des: Vector,
    vel: Vector,
    acc: Vector,
    size: f64,
    width: f64, 
    height: f64,
}

impl Particle {
    pub fn new(width: f64, height: f64, index: usize, size: f64, des: Vector) -> Particle {
        Particle {
            index,
            pos: des.clone(),
            des,
            size,
            width,
            height,
            vel: Vector {x:0., y: 0.},
            acc: Vector {x:0., y: 0.},
        }
    }


    fn gravitas(&mut self, point: Vector) {
        let mut dist = sub_vectors(point, self.pos);
        let mag = dist.mag();
        let force = mapping(mag.abs(), 0., 100., MAX_FORCE / 3., 0.);
        dist.set_mag(force * mag.signum());
        self.acc.add(dist);
    }

    pub fn repellent(&mut self, point: Vector) {
        let mut dist = sub_vectors(point, self.pos);
        let mag = dist.mag();
        let force = mapping(mag.abs(), 0., 100., MAX_FORCE * 50., 0.);
        dist.set_mag(force * mag.signum());
        self.acc.sub(dist);
    }

    fn fate(&mut self) {
        let mut dist = sub_vectors(self.des, self.pos);
        let mag = dist.mag();
        let force = mapping(mag.abs(), 0., 100., 0., MAX_FORCE);
        dist.set_mag(force * mag.signum());
        self.acc.add(dist);
    }

    fn movement(&mut self) {
        self.vel.add(self.acc);
        self.vel.set_mag(f64::min(self.vel.mag(), MAX_SPEED));
        

        if self.pos.x < 0. || self.pos.x + self.size > self.width {
            self.vel.x = -self.vel.x;
        }

        if self.pos.y < 0. || self.pos.y + self.size > self.height {
            self.vel.y = -self.vel.y;
        }
        
        self.pos.add(self.vel);
    }

    fn fraction(&mut self) {
        let mag = self.vel.mag();

        if mag.abs() > 0. {
            self.vel.set_mag((mag.abs() - (mag.abs() / 10.)) * mag.signum());
        }
        
        self.acc.x = 0.;
        self.acc.y = 0.;
    }


    pub fn run(&mut self, point: Vector) -> (usize, Vector) {
        self.gravitas(point);
        self.fate();
        self.movement();
        self.fraction();
        (self.index, self.pos)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping() {
        let mut p = Particle::new(500, 500, 1, 50., Vector {x:100., y:100.});
        p.run(Vector{x:50., y:50.});
    }
}