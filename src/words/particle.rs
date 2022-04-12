use rand::Rng;
use crate::utils::maths::{Vector, mapping, sub_vectors};

const MAX_SPEED: f64 = 10.;
const MAX_FORCE: f64 = 1.;


// pub struct Particle {
//     index: usize,
//     pos: Vector,
//     des: Vector,
//     vel: Vector,
//     acc: Vector,
//     size: f64,
//     width: f64, 
//     height: f64,
// }

// impl Particle {
//     pub fn new(width: f64, height: f64, index: usize, size: f64, des: Vector) -> Particle {
//         Particle {
//             index,
//             pos: des.clone(),
//             des,
//             size,
//             width,
//             height,
//             vel: Vector {x:0., y: 0.},
//             acc: Vector {x:0., y: 0.},
//         }
//     }


//     fn gravitas(&mut self, point: Vector) {
//         let mut dist = sub_vectors(point, self.pos);
//         let mag = dist.mag();
//         let force = mapping(mag.abs(), 0., 100., MAX_FORCE / 3., 0.);
//         dist.set_mag(force * mag.signum());
//         self.acc.add(dist);
//     }

//     pub fn repellent(&mut self, point: Vector) {
//         let mut dist = sub_vectors(point, self.pos);
//         let mag = dist.mag();
//         let force = mapping(mag.abs(), 0., 100., MAX_FORCE * 50., 0.);
//         dist.set_mag(force * mag.signum());
//         self.acc.sub(dist);
//     }

//     fn fate(&mut self) {
//         let mut dist = sub_vectors(self.des, self.pos);
//         let mag = dist.mag();
//         let force = mapping(mag.abs(), 0., 100., 0., MAX_FORCE);
//         dist.set_mag(force * mag.signum());
//         self.acc.add(dist);
//     }

//     fn movement(&mut self) {
//         self.vel.add(self.acc);
//         self.vel.set_mag(f64::min(self.vel.mag(), MAX_SPEED));
        

//         if self.pos.x < 0. || self.pos.x + self.size > self.width {
//             self.vel.x = -self.vel.x;
//         }

//         if self.pos.y < 0. || self.pos.y + self.size > self.height {
//             self.vel.y = -self.vel.y;
//         }
        
//         self.pos.add(self.vel);
//     }

//     fn fraction(&mut self) {
//         let mag = self.vel.mag();

//         if mag.abs() > 0. {
//             self.vel.set_mag((mag.abs() - (mag.abs() / 10.)) * mag.signum());
//         }
        
//         self.acc.x = 0.;
//         self.acc.y = 0.;
//     }


//     pub fn run(&mut self, point: Vector) -> (usize, Vector) {
//         self.gravitas(point);
//         self.fate();
//         self.movement();
//         self.fraction();
//         (self.index, self.pos)
//     }
// }



pub struct Particle {
    index: usize,
    size: f64,
    explosion: i32,
    screen: Vector,
    des: Vector,
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl Particle {
    pub fn new(index: usize, screen: Vector, size: f64, des: Vector) -> Particle {
        let mut rng = rand::thread_rng();
        let pos = Vector {
            x: rng.gen_range((des.x as i32 - 100).max(0)..(des.x as i32 + 100).min(screen.x as i32)) as f64,
            y: rng.gen_range((des.y as i32 - 100).max(0)..(des.y as i32 + 100).min(screen.y as i32)) as f64,
        };

        Particle {
            index,
            screen,
            size,
            des,
            pos,
            acc: Vector {x: 0., y: 0.},
            vel: Vector {x: 0., y: 0.},
            explosion: 0,
        }
    }


    pub fn run(&mut self, gravity: Vector, boom: bool, grenade: Vector) -> (usize, Vector) {

        if self.explosion > 0 {
            self.explosion -= 1;

        } else if boom {

            let mut in_explosion = sub_vectors(grenade, self.pos);
            let expo_mag = in_explosion.mag();
            if expo_mag.abs() < 200. {
                self.vel.x = 0.;
                self.vel.y = 0.;
                self.explosion = 30;
                let force = mapping(expo_mag.abs(), 0., 200., MAX_FORCE * 5., 0.);
                in_explosion.set_mag(force * expo_mag.signum() * -1.);
                self.acc.add(in_explosion);
            }

        } else {

            let mut dist = sub_vectors(self.des, self.pos);
            let des_mag = dist.mag();
            let force = mapping(des_mag.abs(), 0., 200., 0., MAX_FORCE);
            dist.set_mag(force * des_mag.signum());
            self.acc.add(dist);


            let mut in_gravity = sub_vectors(gravity, self.pos);
            let gr_mag = in_gravity.mag();
            if gr_mag.abs() < 200. {
                let force = mapping(gr_mag.abs(), 0., 100., MAX_FORCE, 0.);
                in_gravity.set_mag(force * gr_mag.signum());
                self.acc.add(in_gravity);
            } else {
                self.friction();
            }
        }

        self.movement();
        (self.index, self.pos)
    }


    fn movement(&mut self) {

        self.vel.add(self.acc);
        if self.pos.x < 0. || (self.screen.x - self.pos.x < self.size && self.screen.x > self.pos.x) {
            self.vel.x = -self.vel.x;
        }
        if self.pos.y < 0. || (self.screen.y - self.pos.y < self.size && self.screen.y > self.pos.y) {
            self.vel.y = -self.vel.y
        }

        
        self.pos.add(self.vel);
        self.acc.x = 0.;
        self.acc.y = 0.;
    }

    fn friction(&mut self) {
        let mag = self.vel.mag();
        if mag > 0. {
            self.vel.set_mag(mag - mag / 10.);
        }
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