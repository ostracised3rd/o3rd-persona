// use super::helpers::{GridGenerator, Point};
use wasm_bindgen::prelude::*;
use crate::maze_runner::generators;



#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}



trait Runner {
    fn run(&mut self);
}


pub struct Breadth {
    a: i32
}

impl Runner for Breadth {
    fn run(&mut self) {

    }
}

pub struct Depth {
    a: i32
}

impl Runner for Depth {

    fn run(&mut self) {

    }
}

pub struct AStar {
    a: i32
}

impl Runner for AStar {
    fn run(&mut self) {

    }
}

#[wasm_bindgen]
pub enum Runners {
    BREADTH,
    ASTAR,
    DEPTH,
}


#[wasm_bindgen]
pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<usize>,
    runner: Runners,
    runner_grid: Vec<Vec<Vec<Point>>>
}


#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize, height: usize, runner: Runners) -> Maze {
        let g = generators::division::division(width, height);
        Maze {
            width,
            height,
            grid: to_point(&g),
            runner,
            runner_grid: runner_grid(&g)
        }
    }

    pub fn get_grid(&self) -> Vec<usize> {
        self.grid.clone()
    }
}


fn to_point(grid: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut g = Vec::<usize>::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &1 {
                g.push(x);
                g.push(y);
            }
        }
    }

    g
}

pub fn runner_grid(g: &Vec<Vec<u8>>) -> Vec<Vec<Vec<Point>>> {
    let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

    // println!("{:?}", self.grid);

    for (i, row) in g.iter().enumerate() {
        let mut new_row: Vec<Vec<Point>> = Vec::new();
        for (j, p) in row.iter().enumerate() {
            // println!("i:{}, j:{}, p:{}", i, j, p);
            new_row.push(
                if p == &0 {
                    Vec::<Point>::new()
                } else {
                    neighbors(&g, j, i)
                }    
            )
        } 

        grid.push(new_row);
    }

    grid
}

fn neighbors(g: &Vec<Vec<u8>>, xp: usize, yp: usize) -> Vec<Point> {
    let mut ns: Vec<Point> = Vec::new();

    for y in [yp-1, yp+1] {
        // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
        if g[y][xp] == 1 {
            ns.push(Point{x: xp, y})
        }
    }

    for x in [xp-1, xp+1] {
        // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
        if g[yp][x] == 1 {
            ns.push(Point{x, y:yp})
        }
    }

    ns
}

// impl Maze {
//     pub fn new(width: usize, height: usize) -> Maze {
//         let grid = generators::division::division(width, height);
//         Maze {
//             width,
//             height,
//             grid: grid.flat,
//         }
//     }

// pub fn runner_grid(&self) -> Vec<Vec<Vec<Point>>> {
//     let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

//     // println!("{:?}", self.grid);

//     for (i, row) in self.grid.iter().enumerate() {
//         let mut new_row: Vec<Vec<Point>> = Vec::new();
//         for (j, p) in row.iter().enumerate() {
//             // println!("i:{}, j:{}, p:{}", i, j, p);
//             new_row.push(
//                 if p == &0 {
//                     Vec::<Point>::new()
//                 } else {
//                     self.neighbors(j, i)
//                 }    
//             )
//         } 

//         grid.push(new_row);
//     }

//     grid
// }


//     fn neighbors(&self, xp: usize, yp: usize) -> Vec<Point> {
//         let mut ns: Vec<Point> = Vec::new();

//         for y in [yp-1, yp+1] {
//             // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
//             if self.grid[y][xp] == 1 {
//                 ns.push(Point{x: xp, y})
//             }
//         }

//         for x in [xp-1, xp+1] {
//             // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
//             if self.grid[yp][x] == 1 {
//                 ns.push(Point{x, y:yp})
//             }
//         }

//         ns
//     }

//     pub fn get_start(&self) -> Option<Point> {
//         for (i, p) in self.grid[1].iter().enumerate() {
//             if p == &1 {
//                 return Some(Point{x:i, y:1})
//             }
//         }

//         None
//     }

//     pub fn get_end(&self) -> Option<Point> {
//         let l = self.grid.len();
//         for (i, p) in self.grid[l-2].iter().rev().enumerate() {
//             if p == &1 {
//                 return Some(Point{x:l-1-i, y:l-2})
//             }
//         }

//         None
//     }
// }
