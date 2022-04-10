

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {

    pub fn add(a: Vector, b: Vector) -> Vector {
        Vector { x: (a.x + a.x), y: (a.y + b.y) }
    }

    pub fn sub(a: Vector, b: Vector) -> Vector {
        Vector { x: (a.x - a.x), y: (a.y - b.y) }
    }

    pub fn mag(a: Vector, b: Vector) -> f64 {
        (((a.x + b.x).pow(2) + (a.y + b.y).pow(2)) as f64).sqrt()
    }

    pub fn iadd(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn imag(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    

    // pub fn isub(&mut self, other: Vector) {
    //     self.x -= other.x;
    //     self.y -= other.y;
    // }

    // pub fn imult(&mut self, scaler: i32) {
    //     self.x *= scaler;
    //     self.y *= scaler;
    // }

    // pub fn idiv(&mut self, scaler: i32) {
    //     self.x /= scaler;
    //     self.y /= scaler;
    // }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub start: Vector,
    pub end: Vector,
}