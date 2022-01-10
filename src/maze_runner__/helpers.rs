#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}


pub type GridGenerator = fn(usize, usize) -> Vec<Vec<u8>>;

