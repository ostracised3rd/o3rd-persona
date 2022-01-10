// use rand::seq::SliceRandom;
// use rand::Rng;
// use wasm_bindgen::prelude::*;
// use super::runner::{Point, BreadthFirst};

// fn get_grid(width: usize, height: usize) -> (usize, usize, Vec<Vec<u8>>) {
//     // let h = (2 * height) + 1;
//     // let w = (2 * width) + 1;
//     let mut grid: Vec<Vec<u8>> = Vec::new();

//     for i in 0..height {
//         let mut row: Vec<u8> = Vec::new();
//         for j in 0..width {
//             row.push(if i == 0 || i == height-1 || j == 0 || j == width-1 {0} else {1});
//         }
//         grid.push(row);
//     }

//     (height, width, grid)
// }


// fn division(width: usize, height: usize) -> Vec<Vec<u8>> {
//     let (h, w, mut grid) = get_grid(width, height);

//     const VERTICAL: usize = 0;
//     const HORIZONTAL: usize = 1;

//     let mut region_stack: Vec<((usize, usize), (usize, usize))> = Vec::new();
//     region_stack.push(((1, 1), (h-2, w-2)));

//     let mut rng = rand::thread_rng();
//     while region_stack.len() > 0 {
//         let current_region = region_stack[region_stack.len()-1];
//         region_stack = Vec::from(&region_stack[0..region_stack.len()-1]);
//         let min_y = (current_region.0).0;
//         let max_y = (current_region.1).0;
//         let min_x = (current_region.0).1;
//         let max_x = (current_region.1).1;
//         let height = max_y as i32 - min_y as i32 + 1;
//         let width = max_x as i32 - min_x as i32 + 1;
    
//         if height <= 1 || width <= 1 {continue;}

//         let cut_direction: usize;
//         if width < height {
//             cut_direction = HORIZONTAL;
//         } else if width > height {
//             cut_direction = VERTICAL;
//         } else {
//             if width == 2 {continue;}
//             cut_direction = *[VERTICAL, HORIZONTAL].choose(&mut rand::thread_rng()).unwrap();
//         }

//         let cut_length = [height, width][(cut_direction + 1) % 2];
//         if cut_length < 3 {continue;}

//         let r = rng.gen_range(0..cut_length);
//         let cut_pos = if r % 2 == 0 {(r+1) as usize} else {r as usize};
//         let r = rng.gen_range(1..[height, width][cut_direction]);
//         let door_pos = if r % 2 == 0 {r as usize} else {(r-1) as usize};

//         if cut_direction == VERTICAL {
//             for row in min_y..max_y+1 {
//                 grid[row][min_x + cut_pos] = 0;
//             }

//             grid[min_y+door_pos][min_x+cut_pos] = if min_y+door_pos == 0 || 
//                                                     min_x+cut_pos == 0   ||
//                                                     min_x+cut_pos == w-1 || 
//                                                     min_y+door_pos == h-1 {0} else {1};

//             region_stack.push(((min_y, min_x), (max_y, min_x + cut_pos - 1)));
//             region_stack.push(((min_y, min_x + cut_pos + 1), (max_y, max_x)));

//         } else {
//             for col in min_x..max_x+1 {
//                 grid[min_y+cut_pos][col] = 0;
//             }

//             grid[min_y+cut_pos][min_x+door_pos]  = if min_y+cut_pos == 0 || 
//                                                     min_x+door_pos == 0  ||
//                                                     min_y+cut_pos == h-1 ||
//                                                     min_x+door_pos == w-1 {0} else {1};

//             region_stack.push(((min_y, min_x), (min_y + cut_pos - 1, max_x)));
//             region_stack.push(((min_y + cut_pos + 1, min_x), (max_y, max_x)));
//         }
//     }

//     grid
// }



// #[wasm_bindgen]
// pub struct Maze {
//     grid: Vec<Vec<u8>>,
// }


// #[wasm_bindgen]
// impl Maze {
    // pub fn new(width: usize, height: usize) -> Maze {
    //     Maze {
    //         grid: division(width, height),
    //     }
    // }

    // pub fn matrix(&self) -> Vec<usize> {
    //     let mut ways = Vec::<usize>::new();

    //     for (y, row) in self.grid.iter().enumerate() {
    //         for (x, cell) in row.iter().enumerate() {
    //             if cell == &1 {
    //                 ways.push(x);
    //                 ways.push(y);
    //             }
    //         }
    //     }

    //     ways
    // }

    // pub fn breadth_first(&self) -> BreadthFirst {
    //     BreadthFirst::new(self.graph(), self.start().unwrap(), self.end().unwrap())
    // }
// }


impl Maze {
    fn graph(&self) -> Vec<Vec<Vec<Point>>> {
        let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

        for (i, row) in self.grid.iter().enumerate() {
            let mut new_row: Vec<Vec<Point>> = Vec::new();
            for (j, p) in row.iter().enumerate() {
                // println!("i:{}, j:{}, p:{}", i, j, p);
                new_row.push(
                    if p == &0 {
                        Vec::<Point>::new()
                    } else {
                        self.neighbors(j, i)
                    }    
                )
            } 

            grid.push(new_row);
        }

        grid
    }

    fn neighbors(&self, xp: usize, yp: usize) -> Vec<Point> {
        let mut ns: Vec<Point> = Vec::new();

        for y in [yp-1, yp+1] {
            // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
            if self.grid[y][xp] == 1 {
                ns.push(Point{x: xp, y})
            }
        }

        for x in [xp-1, xp+1] {
            // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
            if self.grid[yp][x] == 1 {
                ns.push(Point{x, y:yp})
            }
        }

        ns
    }

    fn start(&self) -> Option<Point> {
        for (i, p) in self.grid[1].iter().enumerate() {
            if p == &1 {
                return Some(Point{x:i, y:1})
            }
        }

        None
    }

    fn end(&self) -> Option<Point> {
        let l = self.grid.len();
        for (i, p) in self.grid[l-2].iter().rev().enumerate() {
            if p == &1 {
                return Some(Point{x:l-1-i, y:l-2})
            }
        }

        None
    }

    // pub fn cells(&self) -> *const u8 {
    //     self.grid.as_ptr()
    // }
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

// pub fn runner_grid(g: &Vec<Vec<u8>>) -> Vec<Vec<Vec<Point>>> {
//     let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

//     // println!("{:?}", self.grid);

//     for (i, row) in g.iter().enumerate() {
//         let mut new_row: Vec<Vec<Point>> = Vec::new();
//         for (j, p) in row.iter().enumerate() {
//             // println!("i:{}, j:{}, p:{}", i, j, p);
//             new_row.push(
//                 if p == &0 {
//                     Vec::<Point>::new()
//                 } else {
//                     neighbors(&g, j, i)
//                 }    
//             )
//         } 

//         grid.push(new_row);
//     }

//     grid
// }

// fn neighbors(g: &Vec<Vec<u8>>, xp: usize, yp: usize) -> Vec<Point> {
//     let mut ns: Vec<Point> = Vec::new();

//     for y in [yp-1, yp+1] {
//         // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
//         if g[y][xp] == 1 {
//             ns.push(Point{x: xp, y})
//         }
//     }

//     for x in [xp-1, xp+1] {
//         // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
//         if g[yp][x] == 1 {
//             ns.push(Point{x, y:yp})
//         }
//     }

//     ns
// }


// trait Runner {
//     fn run(&mut self);
// }


// pub struct Breadth {
//     a: i32
// }

// impl Runner for Breadth {
//     fn run(&mut self) {

//     }
// }

// pub struct Depth {
//     a: i32
// }

// impl Runner for Depth {

//     fn run(&mut self) {

//     }
// }

// pub struct AStar {
//     a: i32
// }

// impl Runner for AStar {
//     fn run(&mut self) {

//     }
// }

// #[wasm_bindgen]
// pub enum Runners {
//     BREADTH,
//     ASTAR,
//     DEPTH,
// }
