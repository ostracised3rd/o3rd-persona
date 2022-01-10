use wasm_bindgen::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
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

    fn draw(&self, steps: &mut Vec<usize>)  {
        // let mut steps = Vec::<usize>::new();
        steps.push(self.steps.len() * 2);
        for s in self.steps.iter() {
            steps.push(s.x);
            steps.push(s.y);
        }

        // steps
    }
}


pub struct BreadthFirst {
    graph: Vec<Vec<Vec<Point>>>,
    end: Point,
    runners: Vec<BFRunner>,
    finished: bool
}


impl BreadthFirst {
    pub fn new(graph: Vec<Vec<Vec<Point>>>, start: Point, end: Point) -> Self {
        BreadthFirst {
            graph,
            end,
            finished: false,
            runners: Vec::from([BFRunner::new(start)]),
        }
    }
}


impl BreadthFirst {
    pub fn run(&mut self) {
        if self.finished {return}

        let mut new_runners: Vec<BFRunner> = Vec::new();

        for runner in self.runners.iter_mut() {

            if runner.current == self.end {
                new_runners = Vec::new();
                new_runners.push(runner.clone());

                self.finished = true;
                break;
            }
            
            let step = &self.graph[runner.current.y][runner.current.x];

            for s in step {
                if  !runner.steps.contains(s) {
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

        self.runners = new_runners;
    }

    pub fn draw(&self) -> Vec<usize> {
        let mut steps = Vec::<usize>::new();

        for runner in self.runners.iter() {
            runner.draw(&mut steps);
        }

        steps
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }
}









// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Point {
//     pub x: usize,
//     pub y: usize,
// }







// struct Graph {
//     // root: Node
// }


// impl Graph {
//     fn new(data: Vec<Vec<u8>>) -> Self {
//         Graph { }
//     }

//     fn find_nodes(data: Vec<Vec<u8>>) {
//         let mut nodes = Vec::<(usize, usize, (u8, u8, u8, u8))>::new();
//         for (x, row) in data.iter().enumerate() {
//             for (y, cell) in row.iter().enumerate() {
//                 if cell == &1 {
//                     if (row[y-1] as i8 - data[x-1][y] as i8 + row[y+1] as i8 - data[x+1][y] as i8).abs() != 2 {
//                         //    1
//                         //  0   2
//                         //    3
//                         nodes.push((x, y, (row[y-1], data[x-1][y], row[y+1], data[x+1][y])));
//                     }
//                 }
//             }
//         }
//     }
// }


// struct Edge {
//     left: Rc<Node>,
//     right: Rc<Node>,
// }


// struct Node {
//     coords: (usize, usize),
//     children: Vec<Rc<Node>>
// }


// impl Node {
//     fn new(coords: (usize, usize), data: &[usize]) -> Self {

//         Node { coords, children: Vec::new() }
//     }

//     fn find_children(&mut self, data: &[usize]) {}
// }
