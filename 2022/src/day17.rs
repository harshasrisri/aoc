use enum_iterator::{all, Sequence};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Clone)]
enum Directions {
    Left,
    Right,
    Down,
}

impl Directions {
    fn from(input: &str) -> impl Iterator<Item = Directions> + '_ {
        Itertools::intersperse(
            input.chars().cycle().map(|c| match c {
                '<' => Directions::Left,
                '>' => Directions::Right,
                c => panic!("invalid input - {c}"),
            }),
            Directions::Down,
        )
    }
}

struct Chamber {
    space: HashSet<Point>,
    width: usize,
    tallest: usize,
}

impl Chamber {
    fn new(width: usize) -> Self {
        Chamber {
            space: HashSet::new(),
            width,
            tallest: 0,
        }
    }

    fn contains(&self, point: Point) -> bool {
        point.row == 0
            || point.col == 0
            || point.col == self.width + 1
            || self.space.contains(&point)
    }

    fn insert(&mut self, shape: Shape) {
        let shape_tallest = shape
            .points
            .iter()
            .map(|Point { row, col: _ }| row)
            .max()
            .unwrap();
        self.tallest = self.tallest.max(*shape_tallest);
        self.space.extend(shape.points);
    }
}

#[derive(Clone, PartialEq, Sequence)]
enum ShapeType {
    HBar,
    Plus,
    VBar,
    LeftL,
    Square,
}

struct Shape {
    points: Vec<Point>,
    shape_type: ShapeType,
}

impl Shape {
    fn from_type_in_chamber(shape_type: ShapeType, chamber: &Chamber) -> Shape {
        let (srow, scol) = (chamber.tallest + 4, 3);
        let points = match shape_type {
            ShapeType::HBar => [(0, 0), (0, 1), (0, 2), (0, 3)]
                .into_iter()
                .map(|(r, c)| Point {
                    row: srow + r,
                    col: scol + c,
                })
                .collect(),
            ShapeType::Plus => [(1, 0), (2, 1), (1, 1), (0, 1), (1, 2)]
                .into_iter()
                .map(|(r, c)| Point {
                    row: srow + r,
                    col: scol + c,
                })
                .collect(),
            ShapeType::VBar => [(0, 0), (1, 0), (2, 0), (3, 0)]
                .into_iter()
                .map(|(r, c)| Point {
                    row: srow + r,
                    col: scol + c,
                })
                .collect(),
            ShapeType::LeftL => [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
                .into_iter()
                .map(|(r, c)| Point {
                    row: srow + r,
                    col: scol + c,
                })
                .collect(),
            ShapeType::Square => [(0, 0), (1, 0), (0, 1), (1, 1)]
                .into_iter()
                .map(|(r, c)| Point {
                    row: srow + r,
                    col: scol + c,
                })
                .collect(),
        };
        Shape { points, shape_type }
    }

    fn move_left(self, chamber: &Chamber) -> Self {
        let cant_move = self.points.iter().any(|Point { row, col }| {
            chamber.contains(Point {
                row: *row,
                col: col.saturating_sub(1),
            })
        });

        if cant_move {
            self
        } else {
            Shape {
                points: self
                    .points
                    .into_iter()
                    .map(|Point { row, col }| Point {
                        row,
                        col: col.saturating_sub(1),
                    })
                    .collect(),
                shape_type: self.shape_type,
            }
        }
    }

    fn move_right(self, chamber: &Chamber) -> Self {
        let cant_move = self.points.iter().any(|Point { row, col }| {
            chamber.contains(Point {
                row: *row,
                col: col + 1,
            })
        });

        if cant_move {
            self
        } else {
            Shape {
                points: self
                    .points
                    .into_iter()
                    .map(|Point { row, col }| Point { row, col: col + 1 })
                    .collect(),
                shape_type: self.shape_type,
            }
        }
    }

    fn move_down(self, chamber: &Chamber) -> Result<Self, Self> {
        let cant_move = self.points.iter().any(|Point { row, col }| {
            chamber.contains(Point {
                row: row.saturating_sub(1),
                col: *col,
            })
        });

        if cant_move {
            Err(self)
        } else {
            Ok(Shape {
                points: self
                    .points
                    .into_iter()
                    .map(|Point { row, col }| Point {
                        row: row.saturating_sub(1),
                        col,
                    })
                    .collect(),
                shape_type: self.shape_type,
            })
        }
    }

    fn print(&self) {
        let (srow, scol) = self.points.iter().fold(
            (usize::MAX, usize::MAX),
            |(srow, scol), Point { row, col }| (srow.min(*row), scol.min(*col)),
        );
        for row in [3, 2, 1, 0] {
            for col in 0..4 {
                if self.points.contains(&Point {
                    row: row + srow,
                    col: col + scol,
                }) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut directions = Directions::from(input);
    let mut chamber = Chamber::new(7);

    for shape_type in all::<ShapeType>().cycle().take(2022) {
        let mut shape = Shape::from_type_in_chamber(shape_type, &chamber);
        // shape.print();
        for dir in directions.by_ref() {
            shape = match dir {
                Directions::Left => shape.move_left(&chamber),
                Directions::Right => shape.move_right(&chamber),
                Directions::Down => match shape.move_down(&chamber) {
                    Ok(shape) => shape,
                    Err(shape) => {
                        chamber.insert(shape);
                        break;
                    }
                },
            };
        }
    }
    (chamber.tallest, 0)
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(run(input), (3068, 0));
}
