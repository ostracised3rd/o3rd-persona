use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}


type Letter = fn(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line>;


pub fn text_to_line(start: Point, width:i16, height:i16, text: &str) -> (Vec<Line>, i16) {

    let mut letters: HashMap<char, Letter> = HashMap::new();
    letters.insert('A', a);
    letters.insert('B', b);
    letters.insert('C', c);
    letters.insert('D', d);
    letters.insert('E', e);
    letters.insert('F', f);
    letters.insert('G', g);
    letters.insert('H', h);
    letters.insert('I', i);
    letters.insert('J', j);
    letters.insert('K', k);
    letters.insert('L', l);
    letters.insert('M', m);
    letters.insert('N', n);
    letters.insert('O', o);
    letters.insert('P', p);
    letters.insert('Q', q);
    letters.insert('R', r);
    letters.insert('S', s);
    letters.insert('T', t);
    letters.insert('U', u);
    letters.insert('V', v);
    letters.insert('W', w);
    letters.insert('X', x);
    letters.insert('Y', y);
    letters.insert('Z', z);

    let (x, y) = if width > height {(15, 12)} else {(12, 15)};
    let offset = Point{x: width / x, y: height/y};
    let start = Point{x: start.x + offset.x, y: start.y + offset.y};

    let screen_w = width - (offset.x * 2);
    let screen_h = height - (offset.y * 2);

    let text_lines: Vec<&str> = text.lines().map(|x| x.trim()).collect();
    let size = text_lines.iter().map(|x| x.chars().count()).max().unwrap() as i16;

    let font_w = screen_w / size;
    let font_h = screen_h / size;

    let space = font_w / 4;
    let line_h = font_h / 4;

    let font_w = font_w - (space * 2);
    let font_h = font_h - line_h;


    let mut w = start.x;
    let mut h = start.y;
    let mut lines = Vec::<Line>::new();

    for line in text_lines {
        for letter in line.chars() {
            w += font_w;
            if letter == ' ' {continue;}
            lines.extend(letters[&letter](&w, &h, &font_w, &font_h));
            w += space;
        }

        w = start.x;
        h += font_h + line_h;
    }

    return (lines, font_w / 10)
}



fn a(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start: Point{x:*sx, y: *sy},     end: Point{x:*sx+w, y: *sy}},
        Line{start: Point{x:*sx, y: *sy},     end: Point{x:*sx, y: *sy+h}},
        Line{start: Point{x:*sx+w, y: *sy},   end: Point{x:*sx+w, y: *sy+h}},
        Line{start: Point{x:*sx, y: *sy+h/2}, end: Point{x:*sx+w, y: *sy+h/2}},
    ]
} 


fn b(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},   end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx+w, y: *sy}, end:Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn c(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},   end: Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn d(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {

    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx, y: *sy+h},       end: Point{x:*sx+w, y: *sy+h/2}},

    ]
} 


fn e(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx, y: *sy+h}},
    ]
} 


fn f(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy+h/2}},
    ]
} 


fn g(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx+w, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx, y: *sy+h}},
    ]
} 


fn h(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end:Point{x:*sx+w, y: *sy+h/2}},
    ]
} 


fn i(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx+w/2, y: *sy},   end:Point{x:*sx+w/2, y: *sy+h}},
    ]
} 


fn j(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx, y: *sy+h}},
    ]
} 


fn k(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn l(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},       end: Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn m(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w/2, y: *sy+h}},
        Line{start:Point{x:*sx+w/2, y: *sy+h}, end:Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx+w, y: *sy}},
    ]
} 


fn n(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},     end:Point{x:*sx+w, y: *sy+h},},
    ]
} 


fn o(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},   end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy},   end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy+h}, end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy}, end:Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn p(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},   end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h/2}},
    ]
} 


fn q(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},   end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy},   end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy+h}, end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy}, end:Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn r(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},   end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
    ]
} 


fn s(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h/2}},
        Line{start:Point{x:*sx, y: *sy+h/2},   end: Point{x:*sx+w, y: *sy+h/2}},
        Line{start:Point{x:*sx+w, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx, y: *sy+h}},
    ]
} 


fn t(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx+w/2, y: *sy},       end: Point{x:*sx+w/2, y: *sy+h}},
    ]
} 


fn u(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx, y: *sy+h}},
    ]
} 


fn v(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w/2, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx+w/2, y: *sy+h}},
    ]
} 


fn w(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},       end: Point{x:*sx+w/2, y: *sy+h/2}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy+h},   end:Point{x:*sx+w, y: *sy}},
    ]
} 


fn x(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line>{
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy+h}},
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx, y: *sy+h}},
    ]
} 


fn y(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w/2, y: *sy+h/2}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2}, end:Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx+w/2, y: *sy+h/2}, end:Point{x:*sx+w/2, y: *sy+h}},
    ]
} 


fn z(sx: &i16, sy: &i16, w: &i16, h: &i16)  -> Vec<Line> {
    return vec![
        Line{start:Point{x:*sx, y: *sy},       end: Point{x:*sx+w, y: *sy}},
        Line{start:Point{x:*sx+w, y: *sy},       end: Point{x:*sx, y: *sy+h}},
        Line{start:Point{x:*sx, y: *sy+h},   end: Point{x:*sx+w, y: *sy+h}},
    ]
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn overlapping() {
        let text = "HELLO";

        let data = text_to_line(Point{x:0, y: 0}, 500, 300, text);

        println!("{:?}", data);
    }
}