use super::alphabet::{Point, Line, text_to_line};


pub struct Particle {
    size: i16,
    pos: Point,
    des: Point,
    vel: Point,
    acc: Point,
}

impl Particle {
    pub fn new(size: i16, pos: Point, des: Point) -> Particle {
        Particle {
            size, pos, des,
            vel: Point { x: 0, y: 0},
            acc: Point { x: 0, y: 0},
        }
    }
}

pub fn get_points(lines: Vec<Line>, size: i16) -> Vec<u16> {
    let mut points = Vec::<u16>::new();

    for line in lines {
        let mut px = line.start.x as f64;
        let mut py = line.start.y as f64;
        let lx = (line.end.x - line.start.x) as f64;
        let ly = (line.end.y - line.start.y) as f64;
        let mag = f64::sqrt((lx * lx) + (ly * ly));
        let vx = (lx as f64 / mag) * size as f64;
        let vy = (ly as f64 / mag) * size as f64;

        while lx.abs() >= (px - line.start.x as f64).abs() && 
          ly.abs() >= (py - line.start.y as f64).abs()
        {   
            
            points.push(px as u16);
            points.push(py as u16);
            px += vx;
            py += vy; 
        }
    }

    return points
}


pub fn storm(lines: Vec<Line>, size: i16) -> (Vec<u16>, Vec<Particle> ){
    let mut particles = Vec::<Particle>::new();
    let mut points = Vec::<u16>::new();

    for line in lines {
        let p = line_to_particle(line, size);
        for point in p {
            points.push(point.pos.x as u16);
            points.push(point.pos.y as u16);
            particles.push(point);
        }
    }

    return (points, particles)
}


fn line_to_particle(line: Line, size: i16) -> Vec<Particle> {
    let mut particles = Vec::<Particle>::new();


    let mut px = line.start.x as f64;
    let mut py = line.start.y as f64;
    let lx = (line.end.x - line.start.x) as f64;
    let ly = (line.end.y - line.start.y) as f64;
    let mag = f64::sqrt((lx * lx) + (ly * ly));
    let vx = (lx as f64 / mag) * size as f64;
    let vy = (ly as f64 / mag) * size as f64;

    
    while lx.abs() >= (px - line.start.x as f64).abs() && 
          ly.abs() >= (py - line.start.y as f64).abs()
    {   
        px += vx;
        py += vy; 
        particles.push(
            Particle::new(
                size, 
                Point {x: px as i16, y: py as i16}, 
                Point {x: px as i16, y: py as i16}
            )
        )
    }
    
    return particles
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn overlapping() {
        let text = "HELLO";

        let (data, s) = text_to_line(Point{x:0, y: 0}, 500, 300, text);

        println!("{:?}, {}", data, s);

        let points = get_points(data, s);
        println!("{:?}", points);
    }
}