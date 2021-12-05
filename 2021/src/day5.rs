use std::{fmt::Display, num::ParseIntError, str::FromStr};

type PRes<T> = Result<T, ParseIntError>;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(" -> ")
            .flat_map(|coord| coord.split(','))
            .map(|n| n.parse::<usize>())
            .collect::<PRes<Vec<usize>>>()?;
        Ok(Line {
            x1: std::cmp::min(coords[0], coords[2]),
            y1: std::cmp::min(coords[1], coords[3]),
            x2: std::cmp::max(coords[0], coords[2]),
            y2: std::cmp::max(coords[1], coords[3]),
        })
    }
}

struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    pub fn with_size(xmax: usize, ymax: usize) -> Self {
        Matrix(vec![vec![0; ymax + 1]; xmax + 1])
    }

    pub fn add_line(&mut self, line: &Line) {
        if line.x1 == line.x2 {
            for y in line.y1..=line.y2 {
                self.0[line.x1][y] += 1;
            }
        } else if line.y1 == line.y2 {
            for x in line.x1..=line.x2 {
                self.0[x][line.y1] += 1;
            }
        } else {
            // TODO: Not handling diagonal lines
        }
    }

    pub fn hotspots(&self) -> usize { 
        self.0.iter().flat_map(|v| v.iter()).filter(|&heat| *heat >= 2).count()
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

    lines.iter().for_each(|line| plane.add_line(line));

    (plane.hotspots(), 0)
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
    assert_eq!(run(input), (5, 0));
}
