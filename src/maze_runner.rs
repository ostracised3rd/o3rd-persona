use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;


fn get_grid(width: usize, height: usize) -> (usize, usize, Vec<Vec<u8>>) {
    // let h = (2 * height) + 1;
    // let w = (2 * width) + 1;
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for i in 0..height {
        let mut row: Vec<u8> = Vec::new();
        for j in 0..width {
            row.push(if i == 0 || i == height-1 || j == 0 || j == width-1 {0} else {1});
        }
        grid.push(row);
    }

    (height, width, grid)
}


fn division(width: usize, height: usize) -> Vec<Vec<u8>> {
    let (h, w, mut grid) = get_grid(width, height);

    const VERTICAL: usize = 0;
    const HORIZONTAL: usize = 1;

    let mut region_stack: Vec<((usize, usize), (usize, usize))> = Vec::new();
    region_stack.push(((1, 1), (h-2, w-2)));

    let mut rng = rand::thread_rng();
    while region_stack.len() > 0 {
        let current_region = region_stack[region_stack.len()-1];
        region_stack = Vec::from(&region_stack[0..region_stack.len()-1]);
        let min_y = (current_region.0).0;
        let max_y = (current_region.1).0;
        let min_x = (current_region.0).1;
        let max_x = (current_region.1).1;
        let height = max_y as i32 - min_y as i32 + 1;
        let width = max_x as i32 - min_x as i32 + 1;
    
        if height <= 1 || width <= 1 {continue;}

        let cut_direction: usize;
        if width < height {
            cut_direction = HORIZONTAL;
        } else if width > height {
            cut_direction = VERTICAL;
        } else {
            if width == 2 {continue;}
            cut_direction = *[VERTICAL, HORIZONTAL].choose(&mut rand::thread_rng()).unwrap();
        }

        let cut_length = [height, width][(cut_direction + 1) % 2];
        if cut_length < 3 {continue;}

        let r = rng.gen_range(0..cut_length);
        let cut_pos = if r % 2 == 0 {(r+1) as usize} else {r as usize};
        let r = rng.gen_range(1..[height, width][cut_direction]);
        let door_pos = if r % 2 == 0 {r as usize} else {(r-1) as usize};

        if cut_direction == VERTICAL {
            for row in min_y..max_y+1 {
                grid[row][min_x + cut_pos] = 0;
            }

            grid[min_y+door_pos][min_x+cut_pos] = if min_y+door_pos == 0 || 
                                                    min_x+cut_pos == 0   ||
                                                    min_x+cut_pos == w-1 || 
                                                    min_y+door_pos == h-1 {0} else {1};

            region_stack.push(((min_y, min_x), (max_y, min_x + cut_pos - 1)));
            region_stack.push(((min_y, min_x + cut_pos + 1), (max_y, max_x)));

        } else {
            for col in min_x..max_x+1 {
                grid[min_y+cut_pos][col] = 0;
            }

            grid[min_y+cut_pos][min_x+door_pos]  = if min_y+cut_pos == 0 || 
                                                    min_x+door_pos == 0  ||
                                                    min_y+cut_pos == h-1 ||
                                                    min_x+door_pos == w-1 {0} else {1};

            region_stack.push(((min_y, min_x), (min_y + cut_pos - 1, max_x)));
            region_stack.push(((min_y + cut_pos + 1, min_x), (max_y, max_x)));
        }
    }

    grid
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}



#[wasm_bindgen]
struct Maze {
    width: usize,
    height: usize,
    grid: Vec<u8>,
    matrix: Vec<Vec<u8>>,
}


impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let matrix = division(width, height);
        Maze { 
            width,
            height,
            grid: matrix.clone().into_iter().flatten().collect(),
            matrix,
        }
    }

    fn repaint(&mut self) {
        self.grid = self.matrix.clone().into_iter().flatten().collect();
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn graph(&self) -> Vec<Vec<Vec<Point>>> {
        let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

        let mut new_row: Vec<Vec<Point>> = Vec::new();
        for (i, cell) in self.grid.iter().enumerate() {
            
            if i > 0 && i % self.width == 0 {
                grid.push(new_row);
                new_row = Vec::new();
            }

            new_row.push(
                if cell == &0 {
                    Vec::<Point>::new()
                } else {
                    self.neighbors(i % (self.width), i / (self.width))
                }    
            )

        }

        grid
    }

    fn neighbors(&self, xp: usize, yp: usize) -> Vec<Point> {
        let mut ns: Vec<Point> = Vec::new();

        for y in [yp-1, yp+1] {
            // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
            if self.grid[self.get_index(y, xp)] == 1 {
                ns.push(Point{x: xp, y})
            }
        }

        for x in [xp-1, xp+1] {
            // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
            if self.grid[self.get_index(yp, x)] == 1 {
                ns.push(Point{x, y:yp})
            }
        }

        ns
    }

    fn start(&mut self) -> Option<Point> {
        for i in (self.width+1)..(2*(self.width)) {
            if self.grid[i] == 1 {
                self.grid[i] = 2;
                return Some(Point{x: i%(self.width), y:1})
            }
        }

        None
    }

    fn end(&mut self) -> Option<Point> {
        let l = self.grid.len();
        for i in ((l - (2 * self.width))..(l - self.width)).collect::<Vec<usize>>().iter().rev() {
            if self.grid[*i] == 1 {
                self.grid[*i] = 2;
                return Some(Point{x:i%(self.width), y:self.height-2})
            }
        }

        None
    }

    fn visited(&mut self, point: &Point) {
        // for p in points {
            self.grid[point.y * self.width + point.x] = 3
        // }
    }

    fn stepping(&mut self, point: &Point) {
        // for p in points {
            self.grid[point.y * self.width + point.x] = 2
        // }
    }

    fn running(&mut self, points: &Vec<Point>, c: u8) {
        for point in points {
            self.grid[point.y * self.width + point.x] = c
        }
    }
}

#[derive(Debug, Clone)]
struct BFRunner {
    current: Point,
    last: Point,
    steps: Vec<Point>,
}

impl BFRunner {
    fn new(start: Point) -> Self {
        BFRunner {
            current: start, 
            last: start, 
            steps: Vec::from([start])
        }
    }
}


#[wasm_bindgen]
pub struct BreadthFirst {
    maze: Maze,
    graph: Vec<Vec<Vec<Point>>>,
    end: Point,
    runners: Vec<BFRunner>,
    finished: bool
}


#[wasm_bindgen]
impl BreadthFirst {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze::new(width, height);
        BreadthFirst {
            graph: maze.graph(),
            end: maze.end().unwrap(),
            finished: false,
            runners: Vec::from([BFRunner::new(maze.start().unwrap())]),
            maze,
        }
    }


    pub fn run_better(&mut self) {
        if self.finished {return}

        for runner in self.runners.iter_mut() {

        } 
    }


    pub fn run(&mut self, c: u8) {
        // horrible execution!
        if self.finished {return}

        let mut new_runners: Vec<BFRunner> = Vec::new();

        for runner in self.runners.iter() {

            if runner.current == self.end {
                new_runners = Vec::new();
                new_runners.push(runner.clone());
                self.finished = true;
                
                break;
            }
            
            let step = &self.graph[runner.current.y][runner.current.x];

            for s in step {
                if !runner.steps.contains(s) {
                    let mut nr = BFRunner {
                        last: runner.current,
                        current: *s,
                        steps: runner.steps.clone(),
                    };

                    nr.steps.push(*s);
                    new_runners.push(nr);
                }
            }
        }

        for runner in new_runners.iter() {
            self.maze.running(&runner.steps, c);
        }

        self.runners = new_runners;
    }

    pub fn cells(&self) -> *const u8 {
        self.maze.grid.as_ptr()
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }

    pub fn get_end(&self) -> Vec<usize> {
        vec![self.end.x, self.end.y]
    }
}



// struct BreadthFirstNext {
//     maze: Maze,
//     runners: Vec<BFRunner>,
//     end: Point,
//     is_done: bool
// }

// impl BreadthFirstNext {
//     fn run(&mut self) {
//         for runner in self.runners.iter_mut() {
//             let steps = self.maze.neighbors(runner.current.x, runner.current.y);


//         }
//     }
// }







struct Node {
    index: usize,
    coords: Point
}


#[wasm_bindgen]
pub struct DepthFirst {
    maze: Maze,
    graph: Vec<Vec<Vec<Point>>>,
    end: Point,
    nodes: Vec<Node>,
    runner: Vec<Point>,
    finished: bool,
}


#[wasm_bindgen]
impl DepthFirst {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze::new(width, height);

        DepthFirst {
            graph: maze.graph(),
            end: maze.end().unwrap(),
            finished: false,
            runner: vec![maze.start().unwrap()],
            nodes: Vec::new(),
            maze
        }
    }

    pub fn run(&mut self) {
        if self.finished {return}

        let current = self.runner.last().unwrap();

        if current == &self.end {
            self.finished = true;
            return
        }

        let steps = &self.graph[current.y][current.x];
        for step in steps {
            if !self.runner.contains(step) {
                self.nodes.push(Node {index: self.runner.len(), coords: *step})
            }
        }

        let next_node = self.nodes.pop().unwrap();

        for _ in next_node.index..self.runner.len() {
            if let Some(point) = self.runner.pop() {
                self.maze.visited(&point);
            }
        }

        self.maze.stepping(&next_node.coords);
        self.runner.push(next_node.coords);
    }

    pub fn cells(&self) -> *const u8 {
        self.maze.grid.as_ptr()
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }

    pub fn get_end(&self) -> Vec<usize> {
        vec![self.end.x, self.end.y]
    }
}


