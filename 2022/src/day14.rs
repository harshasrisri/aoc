use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let (x, y) = input.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn below(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn bot_left(&self) -> Option<Point> {
        self.x.checked_sub(1).map(|x| Point { x, y: self.y + 1 })
    }

    fn bot_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn from_ends(p1: Point, p2: Point) -> Line {
        Line { p1, p2 }
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let (x1, x2) = (self.p1.x.min(self.p2.x), self.p1.x.max(self.p2.x));
        let (y1, y2) = (self.p1.y.min(self.p2.y), self.p1.y.max(self.p2.y));
        (x1..=x2)
            .cartesian_product(y1..=y2)
            .map(|(x, y)| Point { x, y })
    }
}

struct Map {
    occupied: HashSet<Point>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let occupied = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ").tuple_windows().flat_map(|(p1, p2)| {
                    Line::from_ends(Point::from_str(p1), Point::from_str(p2)).points()
                })
            })
            .collect();
        Map { occupied }
    }

    fn abysmal(&self, p: &Point) -> bool {
        self.occupied
            .iter()
            .filter(|m| m.x == p.x)
            // .inspect(|m| eprintln!("comparing {:?} <=> {:?}", m, p))
            .all(|m| m.y < p.y)
    }

    fn add_sand(&mut self) -> Option<Point> {
        let mut new_sand = Point { x: 500, y: 0 };
        loop {
            // eprintln!("Releasing sand at {:?}", new_sand);
            new_sand = match (new_sand.bot_left(), new_sand.below(), new_sand.bot_right()) {
                (None, _, _) => {
                    // eprintln!("Falling off the left edge");
                    return None;
                }
                (Some(bl), bot, br) => {
                    match *[&bl, &bot, &br]
                        .iter()
                        .map(|p| self.occupied.contains(p))
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        [true, true, true] => {
                            // final resting point
                            // eprintln!("Landed at {:?}", new_sand);
                            self.occupied.insert(new_sand.clone());
                            return Some(new_sand);
                        }
                        [false, true, _] => bl,    // continue left
                        [true, true, false] => br, // continue right
                        [_, false, _] => {
                            if self.abysmal(&bot) {
                                // eprintln!("abysmal point: {:?}", new_sand);
                                return None;
                            } else {
                                bot
                            }
                        }
                        _ => {
                            panic!("Shouldn't have run into this case");
                        }
                    }
                }
            }
        }
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut bitmap = Map::from_str(input);
    // bitmap.occupied.iter().for_each(|p| eprintln!("{:?}", p));
    let p1 = (1..).take_while(|_| bitmap.add_sand().is_some()).count();

    (p1, 0)
}

#[test]
fn test() {
    let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
    assert_eq!(run(input), (24, 0));
}
