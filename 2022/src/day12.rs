use std::{collections::HashMap, fmt::Display, ops::Index, str::FromStr};

#[derive(Default, PartialEq, Eq, Hash, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:03?}, {:03?})", self.row, self.col)
    }
}

struct Matrix<T>(Vec<Vec<T>>);

#[derive(Default)]
struct Grid {
    grid: Vec<Vec<isize>>,
    botright: Position,
    start: Position,
    end: Position,
    scores: HashMap<Position, usize>,
}

impl Index<&Position> for Grid {
    type Output = isize;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.grid[index.row][index.col]
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut start = Position { row: 0, col: 0 };
        let mut end = Position { row: 0, col: 0 };

        let grid: Vec<Vec<_>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        'S' => {
                            start = Position { row, col };
                            0
                        }
                        'E' => {
                            end = Position { row, col };
                            27
                        }
                        ch => 1 + ch as isize - 'a' as isize,
                    })
                    .collect()
            })
            .collect();

        let botright = Position {
            row: grid.len(),
            col: grid[0].len(),
        };

        Ok(Grid {
            grid,
            botright,
            start,
            end,
            ..Default::default()
        })
    }
}

impl Grid {
    fn print_scores(&self) {
        print!("{}[2J", 27 as char);
        for row in 0..self.botright.row {
            for col in 0..self.botright.col {
                let pos = Position { row, col };
                // let pr = self.scores.get(&pos).map(|s| format!("{:4}", s).unwrap_or("    ".to_string()));
                let pr = if self.scores.contains_key(&pos) {
                    char::from_u32((self[&pos] + 'a' as isize - 1) as u32).unwrap_or(' ')
                } else {
                    ' '
                };
                print!(" {}", pr);
            }
            println!();
        }
        // std::thread::sleep(Duration::from_millis(50));
    }

    fn neighbours(&self, curr: &Position) -> impl Iterator<Item = Position> {
        let (r, c) = (curr.row as isize, curr.col as isize);
        let (rm, cm) = (self.botright.row as isize, self.botright.col as isize);

        [(r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c)]
            .into_iter()
            .filter_map(move |(r, c)| {
                if r >= 0 && r < rm && c >= 0 && c < cm {
                    Some((r as usize, c as usize))
                } else {
                    None
                }
            })
            .map(|(row, col)| Position { row, col })
    }

    fn traverse(&mut self, curr: &Position) {
        // self.print_scores();
        for neighbour in self.neighbours(&curr) {
            let n_dist = self[&curr] - self[&neighbour];
            if ![-1, 0, 1].contains(&n_dist) {
                continue;
            }

            let curr_score = *self.scores.get(curr).unwrap();
            if neighbour == self.start {
                println!(
                    "curr: {curr}({:03?}), neighbour: {neighbour}({:03?})",
                    curr_score,
                    curr_score + 1
                );
            }

            let neighbour_score = curr_score + 1;
            self.scores
                .entry(neighbour.clone())
                .and_modify(|score| *score = neighbour_score.min(*score))
                .or_insert(neighbour_score);

            if self.scores[&curr] < self.scores[&neighbour] {
                self.traverse(&neighbour);
            }
        }
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut grid = Grid::from_str(input).expect("Invalid input");
    let start = grid.start.clone();
    let end = grid.end.clone();

    grid.scores.insert(end.clone(), 0);
    grid.traverse(&end);
    // eprintln!("grid_size: {}, num_scores: {}", grid.rows * grid.cols, grid.scores.len());

    let p1 = grid.scores.remove(&start).unwrap_or_default();

    (p1, 0)
}

#[test]
fn test() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    assert_eq!(run(input), (31, 0));
}
