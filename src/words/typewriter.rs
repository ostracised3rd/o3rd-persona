use std::collections::HashMap;
use crate::utils::maths::{Line, Vector};

type Letter = fn(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line>;


pub struct Typewriter {
    alphabet: HashMap<char, Letter>
}


impl Typewriter {
    fn new() -> Self {
        Typewriter {
            alphabet: Typewriter::populate()
        }
    }

    fn populate() -> HashMap<char, Letter> {
        let mut alphabet: HashMap<char, Letter> = HashMap::new();
        alphabet.insert('A', a);
        alphabet.insert('B', b);
        alphabet.insert('C', c);
        alphabet.insert('D', d);
        alphabet.insert('E', e);
        alphabet.insert('F', f);
        alphabet.insert('G', g);
        alphabet.insert('H', h);
        alphabet.insert('I', i);
        alphabet.insert('J', j);
        alphabet.insert('K', k);
        alphabet.insert('L', l);
        alphabet.insert('M', m);
        alphabet.insert('N', n);
        alphabet.insert('O', o);
        alphabet.insert('P', p);
        alphabet.insert('Q', q);
        alphabet.insert('R', r);
        alphabet.insert('S', s);
        alphabet.insert('T', t);
        alphabet.insert('U', u);
        alphabet.insert('V', v);
        alphabet.insert('W', w);
        alphabet.insert('X', x);
        alphabet.insert('Y', y);
        alphabet.insert('Z', z);

        return alphabet
    }

    fn quill(&self, letter: &char, sx: &f64, sy: &f64, w: &f64, h: &f64) -> Vec<Line> {
        self.alphabet[letter](sx, sy, w, h)
    }
}


pub fn lettering(start: Vector, end: Vector, text: &str) -> (Vec<Line>, f64) {
    let typewriter = Typewriter::new();
    let text_lines: Vec<&str> = text.lines().map(|x| x.trim()).collect();
    let text_len = text_lines.iter().map(|x| x.chars().count()).max().unwrap() as f64 ;

    println!("{:?} {:?}", start, end);

    let fx = (end.x - start.x) / (4. + text_len + ((text_len-2.)/2.));
    let fy = fx  * 1.5;

    let off_x = start.x + 2.*fx;
    let off_y = ((end.y - start.y) - fy * ((text_lines.len() as f64 * 2. ) - 1.)) / 3.;

    let (mut w, mut h) = (off_x, start.y + 2. * off_y);
    let mut lines = Vec::<Line>::new();

    for line in text_lines {
        for letter in line.chars() {
            
            if letter == ' ' {
                w += fx;
                continue;
            }

            lines.extend(typewriter.quill(&letter, &w, &h, &fx, &fy));
            w += fx + fx/2.;
        }

        w = off_x;
        h += fy + fy / 2.;
    }

    (lines, fx)
}


fn a(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start: Vector{x:*sx, y: *sy},     end: Vector{x:*sx+w, y: *sy}},
        Line{start: Vector{x:*sx, y: *sy},     end: Vector{x:*sx, y: *sy+h}},
        Line{start: Vector{x:*sx+w, y: *sy},   end: Vector{x:*sx+w, y: *sy+h}},
        Line{start: Vector{x:*sx, y: *sy+h/2.}, end: Vector{x:*sx+w, y: *sy+h/2.}},
    ]
} 


fn b(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},     end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end:Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w, y: *sy},     end:Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn c(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},   end: Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn d(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {

    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx, y: *sy+h},       end: Vector{x:*sx+w, y: *sy+h/2.}},

    ]
} 


fn e(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx, y: *sy+h},   end: Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn f(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end: Vector{x:*sx+w, y: *sy+h/2.}},
    ]
} 


fn g(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.},   end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w, y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},    end:Vector{x:*sx+w, y: *sy+h},   },
    ]
} 


fn h(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end:Vector{x:*sx+w, y: *sy+h/2.}},
    ]
} 


fn i(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx+w/2., y: *sy},   end:Vector{x:*sx+w/2., y: *sy+h}},
    ]
} 


fn j(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end: Vector{x:*sx, y: *sy+h}},
    ]
} 


fn k(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start: Vector{x:*sx+w, y: *sy}, end:Vector{x:*sx, y: *sy+h/2.},   },
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end: Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn l(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},       end: Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn m(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w/2., y: *sy+h}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h}, end:Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx+w, y: *sy}, end:Vector{x:*sx+w, y: *sy+h},   },
    ]
} 


fn n(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},     end:Vector{x:*sx+w, y: *sy+h},},
    ]
} 


fn o(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},   end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy},   end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy+h}, end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy}, end:Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn p(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},   end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx, y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h/2.}},
    ]
} 


fn q(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},   end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy},   end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy+h}, end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy}, end:Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn r(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},   end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx, y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
    ]
} 


fn s(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h/2.}},
        Line{start:Vector{x:*sx, y: *sy+h/2.},   end: Vector{x:*sx+w, y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w, y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy+h},   end:Vector{x:*sx, y: *sy+h}},
    ]
} 


fn t(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx+w/2., y: *sy},       end: Vector{x:*sx+w/2., y: *sy+h}},
    ]
} 


fn u(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy+h},   end:Vector{x:*sx, y: *sy+h}},
    ]
} 


fn v(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w/2., y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx+w/2., y: *sy+h}},
    ]
} 


fn w(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},       end: Vector{x:*sx+w/2., y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy+h},   end:Vector{x:*sx+w, y: *sy}},
    ]
} 


fn x(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line>{
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy+h}},
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
    ]
} 


fn y(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w/2., y: *sy+h/2.}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.}, end:Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx+w/2., y: *sy+h/2.}, end:Vector{x:*sx+w/2., y: *sy+h}},
    ]
} 


fn z(sx: &f64, sy: &f64, w: &f64, h: &f64)  -> Vec<Line> {
    return vec![
        Line{start:Vector{x:*sx, y: *sy},       end: Vector{x:*sx+w, y: *sy}},
        Line{start:Vector{x:*sx+w, y: *sy},       end: Vector{x:*sx, y: *sy+h}},
        Line{start:Vector{x:*sx, y: *sy+h},   end: Vector{x:*sx+w, y: *sy+h}},
    ]
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn overlapping() {
        let text = "SOHEIL\n DEVELOPER AND\n SOME OTHER THINGS";

        let text_lines: Vec<&str> = text.lines().map(|x| x.trim()).collect();

        println!("{:?}", &text_lines);

        for line in text_lines.iter() {
            println!("{} {}", line, line.chars().count());
        }

        let size = text_lines.iter().map(|x| x.chars().count()).max().unwrap() as i32;

        println!("{}", size);

        // let _data = lettering(Vector{x:0, y: 0}, Vector{x:500, y: 300}, text);

        // println!("{:?}", data);
    }
}