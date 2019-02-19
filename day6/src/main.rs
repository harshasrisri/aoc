#[macro_use]
extern crate text_io;

use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    r: usize,
    c: usize,
    id: usize,
    inf: bool,
    count: usize,
}

impl Point {
    pub fn new(r: usize, c: usize, id: usize) -> Point {
        Point { r, c, id, inf: false, count: 0 }
    }
    pub fn dist(&self, r: usize, c: usize) -> usize {
        (r as isize - self.r as isize).abs() as usize
            + (c as isize - self.c as isize).abs() as usize
    }
    pub fn set_edge(&mut self, flag: bool) { self.inf |= flag; }
    pub fn increment(&mut self) { self.count += 1; }
}

struct Plane {
    w: usize,
    h: usize,
}

impl Plane {
    pub fn new(w: usize, h: usize) -> Plane { Plane { w, h } }
    pub fn height(&self) -> usize { self.h }
    pub fn width(&self) -> usize { self.w }
    pub fn is_edge(&self, r: usize, c: usize) -> bool {
        r == 0 || c == 0 || r == self.height() - 1 || c == self.width() - 1
    }
}

fn main() {
    let mut points = Vec::new();
    let (mut w, mut h) = (0usize, 0usize);
    for line in BufReader::new(std::io::stdin()).lines().filter_map(|r| r.ok())
    {
        let (r, c): (usize, usize);
        scan!(line.bytes() => "{},{}", c, r);
        w = core::cmp::max(c, w);
        h = core::cmp::max(r, h);
        points.push(Point::new(r, c, points.len()))
    }

    let plane = Plane::new(w + 1, h + 1);
    let mut goodregion = 0;

    for r in 0..plane.height() {                        // Loop over all rows in the plane
        for c in 0..plane.width() {                     // Loop over all cells in a row
            let mut border = false;                     // is this cell a border?
            let mut nearest = 0;                        // ID of the nearest Point to this cell
            let mut mindist = usize::max_value();       // Dist of the current nearest Point
            let mut totaldist = 0;                      // Sum of distances to all Points

            for p in &points {                          // Loop over all points enumerating with indices
                let curdist = p.dist(r, c);             // Manhattan dist b/w cell and Point in question
                totaldist += curdist;
                if curdist < mindist {                  // Note this Point if nearer than current nearest
                    mindist = curdist;
                    nearest = p.id;
                    border = false;
                } else if curdist == mindist {          // Mark as border if cell is nearest to more than one Point
                    border = true;
                }
            }

            if !border {                                // If cell not a border, then update Point's region
                let nearest = &mut points[nearest];
                nearest.increment();
                nearest.set_edge(plane.is_edge(r, c));  // Check if cell is Plane's edge to check region's infinity 
            }

            if totaldist < 10000 {                      // Check if cell is in a good region near to all points
                goodregion += 1;
            }
        }
    }

    let maxarea = points.iter().filter(|p| !p.inf).map(|p| p.count).max().unwrap();

    println!("Part 1: {}, Part 2: {}", maxarea, goodregion);
}
