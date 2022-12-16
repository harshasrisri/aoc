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
    floor: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut floor = 0;
        let occupied = input
            .lines()
            .flat_map(|line| {
                line.split(" -> ").tuple_windows().flat_map(|(p1, p2)| {
                    Line::from_ends(Point::from_str(p1), Point::from_str(p2)).points()
                })
            })
            .inspect(|p| floor = floor.max(p.y))
            .collect();
        Map {
            occupied,
            floor: floor + 2,
        }
    }

    fn contains(&self, p: &Point, floor: bool) -> bool {
        if floor && p.y == self.floor {
            true
        } else {
            self.occupied.contains(p)
        }
    }

    fn abysmal(&self, p: &Point) -> bool {
        self.occupied
            .iter()
            .filter(|m| m.x == p.x)
            // .inspect(|m| eprintln!("comparing {:?} <=> {:?}", m, p))
            .all(|m| m.y < p.y)
    }

    fn add_sand(&mut self, floor: bool) -> Option<Point> {
        let mut new_sand = Point { x: 500, y: 0 };
        if self.contains(&new_sand, floor) {
            return None;
        }
        loop {
            // eprintln!("new sand at {:?}", new_sand);
            new_sand = match (new_sand.bot_left(), new_sand.below(), new_sand.bot_right()) {
                (None, _, _) => {
                    // eprintln!("Falling off the left edge");
                    return None;
                }
                (Some(bl), bot, br) => {
                    match *[&bl, &bot, &br]
                        .iter()
                        .map(|p| self.contains(p, floor))
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        [true, true, true] => {
                            // final resting point
                            eprintln!("Landed at {:?}", new_sand);
                            self.occupied.insert(new_sand.clone());
                            return Some(new_sand);
                        }
                        [false, true, _] => bl,    // continue left
                        [true, true, false] => br, // continue right
                        [_, false, _] => {
                            if !floor && self.abysmal(&bot) {
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

    fn print(&self, floor: bool) {
        let (xmn, xmx, ymn, ymx) = self.occupied.iter().fold(
            (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
            |(xmn, xmx, ymn, ymx), p| (xmn.min(p.x), xmx.max(p.x), ymn.min(p.y), ymx.max(p.y)),
        );
        let map = (xmn..=xmx)
            .into_iter()
            .cartesian_product((ymn..=ymx).into_iter())
            .map(|(x, y)| {
                if self.contains(&Point { x, y }, floor) {
                    'O'
                } else {
                    '.'
                }
            })
            .collect::<Vec<char>>()
            .chunks(xmx - xmn)
            .map(|ch| ch.iter().collect::<String>())
            .join("\n");
        eprintln!("{map}");
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut bitmap = Map::from_str(input);
    // bitmap.occupied.iter().for_each(|p| eprintln!("{:?}", p));
    let p1 = (1..)
        .take_while(|_| bitmap.add_sand(false).is_some())
        .count();
    bitmap.print(false);
    eprintln!("======= Part 2 =======");
    let p2 = (1..)
        .take_while(|_| bitmap.add_sand(true).is_some())
        .count();
    // bitmap.print(true);

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
    assert_eq!(run(input), (24, 93));
}
