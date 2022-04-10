// use super::typewriter::{Line, Vector};

use crate::utils::vector::{Line, Vector};

const MAX_SPEED: i32 = 10;
const MAX_FORCE: i32 = 10;


pub struct Particle {
    index: usize,
    pos: Vector,
    des: Vector,
    vel: Vector,
    acc: Vector,
    size: i32,
}

impl Particle {
    pub fn new(index: usize, size: i32, des: Vector) -> Particle {
        Particle {
            index,
            pos: des.clone(),
            des,
            size,
            vel: Vector {x:0, y: 0},
            acc: Vector {x:0, y: 0},
        }
    }

    pub fn apply_force(&mut self, force: Vector) {
        self.acc.iadd(force);
    }


    pub fn steering(&mut self) {
        let dist = Vector::sub(self.des, self.pos);


    }


    pub fn run(&mut self) -> (usize, Vector) {
        self.steering();

        (self.index, self.pos)
    }
}




#[cfg(test)]
mod tests {
    use crate::words::typewriter::lettering;

    use super::*;


    #[test]
    fn overlapping() {
        let text = "HELLO";

        let (data, s) = lettering(Vector{x:0, y: 0}, Vector{x:500, y: 500}, text);

        println!("{:?}, {}", data, s);
    }
}