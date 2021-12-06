use std::{fmt::Display, num::ParseIntError, str::FromStr};

type PRes<T> = Result<T, ParseIntError>;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    xd: isize,
    yd: isize,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(" -> ")
            .flat_map(|coord| coord.split(','))
            .map(|n| n.parse::<usize>())
            .collect::<PRes<Vec<usize>>>()?;
        let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);
        Ok(Line {
            x1,
            y1,
            x2,
            y2,
            xd: match x1.cmp(&x2) {
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => 1,
            },
            yd: match y1.cmp(&y2) {
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => 1,
            },
        })
    }
}

struct LineIter<'a> {
    line: &'a Line,
    curr: Option<(usize, usize)>,
    ended: bool,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        self.curr = if let Some((cur_x, cur_y)) = self.curr {
            let cur_x = (cur_x as isize + self.line.xd) as usize;
            let cur_y = (cur_y as isize + self.line.yd) as usize;

            if cur_x == self.line.x2 && cur_y == self.line.y2 {
                self.ended = true;
            }

            Some((cur_x, cur_y))
        } else {
            Some((self.line.x1, self.line.y1))
        };

        self.curr
    }
}

impl Line {
    pub fn iter(&self) -> LineIter {
        LineIter {
            line: self,
            curr: None,
            ended: false,
        }
    }

    pub fn h_or_v(&self) -> bool {
        self.xd == 0 || self.yd == 0
    }
}

struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    pub fn with_size(xmax: usize, ymax: usize) -> Self {
        Matrix(vec![vec![0; ymax + 1]; xmax + 1])
    }

    pub fn plot_line(&mut self, line: &Line) {
        for (x, y) in line.iter() {
            self.0[x][y] += 1;
        }
    }

    pub fn hotspots(&self) -> usize {
        self.0
            .iter()
            .flat_map(|v| v.iter())
            .filter(|&heat| *heat >= 2)
            .count()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.0[0].len() {
            for x in 0..self.0.len() {
                write!(f, "{} ", self.0[x][y])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let lines = input
        .lines()
        .map(Line::from_str)
        .collect::<PRes<Vec<_>>>()
        .expect("Failed to parse some line");

    let (xmax, ymax) = lines.iter().fold((0, 0), |(xmax, ymax), line| {
        let xmax = xmax.max(line.x1).max(line.x2);
        let ymax = ymax.max(line.y1).max(line.y2);
        (xmax, ymax)
    });

    let mut plane = Matrix::with_size(xmax, ymax);

    lines
        .iter()
        .filter(|line| line.h_or_v())
        .for_each(|line| plane.plot_line(line));
    let d5p1 = plane.hotspots();

    lines
        .iter()
        .filter(|line| !line.h_or_v())
        .for_each(|line| plane.plot_line(line));
    let d5p2 = plane.hotspots();

    (d5p1, d5p2)
}

#[test]
fn test() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
    assert_eq!(run(input), (5, 12));
}
