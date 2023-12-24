use std::{collections::HashMap, sync::LazyLock};

use geo::{coord, LineString, Polygon, point, Contains};
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

static RUBRIC: LazyLock<HashMap<char, HashMap<Heading, Heading>>> = LazyLock::new(|| {
    let mut rubric: HashMap<char, HashMap<Heading, Heading>> = HashMap::new();
    let pipe_headings: [(char, (Heading, Heading)); 12] = [
        ('|', (Heading::North, Heading::North)),
        ('|', (Heading::South, Heading::South)),
        ('-', (Heading::East, Heading::East)),
        ('-', (Heading::West, Heading::West)),
        ('L', (Heading::South, Heading::East)),
        ('L', (Heading::West, Heading::North)),
        ('J', (Heading::South, Heading::West)),
        ('J', (Heading::East, Heading::North)),
        ('7', (Heading::East, Heading::South)),
        ('7', (Heading::North, Heading::West)),
        ('F', (Heading::North, Heading::East)),
        ('F', (Heading::West, Heading::South)),
    ];

    for (pipe, (incoming, outgoing)) in pipe_headings {
        rubric.entry(pipe)
            .and_modify(|mp| { mp.insert(incoming, outgoing); })
            .or_insert(HashMap::from([(incoming, outgoing)]));
    }
    rubric
});

impl Heading {
    fn next(&self, pipe: char) -> Option<Self> {
        RUBRIC.get(&pipe)?.get(self).copied()
    }

    fn check(&self, pipe: char) -> Option<()> {
        if RUBRIC.get(&pipe)?.contains_key(self) {
            Some(())
        } else {
            None
        }
    }
}

fn walk(input: &[Vec<char>], row: usize, col: usize, heading: Heading, path: &mut Vec<(usize, usize)>) -> Option<usize> {
    let cur_cell = input.get(row).and_then(|rw| rw.get(col))?;
    path.push((row, col));
    if *cur_cell == 'S' {
        return Some(1);
    }
    heading.check(*cur_cell)?;
    let next_heading = heading.next(*cur_cell)?;
    let next_cell = match next_heading {
        Heading::South => (row + 1, col),
        Heading::North => (row.checked_sub(1)?, col),
        Heading::East => (row, col + 1),
        Heading::West => (row, col.checked_sub(1)?),
    };
    walk(input, next_cell.0, next_cell.1, next_heading, path).map(|dist| dist + 1)
}

fn before_walking(input: &[Vec<char>], row: usize, col: usize, heading: Heading, path: &mut Vec<(usize, usize)>) -> Option<usize> {
    path.clear();
    walk(input, row, col, heading, path)
}

fn begin(input: &[Vec<char>], row: usize, col: usize, path: &mut Vec<(usize, usize)>) -> usize {
    before_walking(input, row+1, col, Heading::South, path)
        .or_else(|| before_walking(input, row, col+1, Heading::East, path))
        .or_else(|| row.checked_sub(1).and_then(|row| before_walking(input, row, col, Heading::North, path)))
        .or_else(|| col.checked_sub(1).and_then(|col| before_walking(input, row, col, Heading::West, path)))
        .unwrap()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut start = (0, 0);
    let input = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if let Some(col) = line.find('S') {
                start = (row, col);
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut path = Vec::new();
    let p1 = begin(&input, start.0, start.1, &mut path) / 2;

    path.insert(0, start);
    let lines = path.iter().map(|(x, y)| coord! {x: *x as f32, y: *y as f32}).collect::<LineString<f32>>();
    let poly = Polygon::new(lines, vec![]);

    let p2 = (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|(row, col)| poly.contains(&point!(x: *row as f32, y: *col as f32)))
        .count();

    (p1, p2)
}

#[test]
fn test1() {
    let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
    assert_eq!(run(input).0, 4);
    println!();

    let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    assert_eq!(run(input).0, 8);
    println!();
}

#[test]
fn test2() {
    let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
    assert_eq!(run(input).1, 4);
    println!();

    let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    assert_eq!(run(input).1, 8);
}
