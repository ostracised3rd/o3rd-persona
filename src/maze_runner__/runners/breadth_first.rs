use crate::maze_runner::helpers::Point;
use rand::Rng;


#[derive(Debug, Clone)]
pub struct Agent {
    pub color_index: usize,
    current: Point,
    last: Point,
    pub steps: Vec<Point>,
}


impl Agent {
    fn new(start: Point) -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            color_index: rng.gen_range(0..7),
            current: start, 
            last: start, 
            steps: Vec::from([start])
        }
    }
}

pub struct BreadthFirst {
    
    grid: Vec<Vec<Vec<Point>>>,
    end: Point,
    pub agents: Vec<Agent>,
    finished: bool
}

 


impl BreadthFirst {
    pub fn new(start: Point, end: Point, grid: Vec<Vec<Vec<Point>>>) -> Self {
        BreadthFirst {
            grid,
            end,
            finished: false,
            agents: Vec::from([Agent::new(start)]),
        }
    }

    pub fn run(&mut self) {
        
        if self.finished {
            return
        }

        let mut new_runners: Vec<Agent> = Vec::new();

        for r in self.agents.iter() {

            if r.current == self.end {
                new_runners = Vec::new();
                new_runners.push(r.clone());

                

                self.finished = true;
                break;
            }
            
            let step = &self.grid[r.current.y][r.current.x];

            for s in step {
                if  !r.steps.contains(s) {
                    let mut nr = Agent {
                        color_index: r.color_index,
                        last: r.current,
                        current: *s,
                        steps: r.steps.clone(),
                    };

                    nr.steps.push(*s);
                    new_runners.push(nr);
                }
            }
        }

        self.agents = new_runners;
    }
}
