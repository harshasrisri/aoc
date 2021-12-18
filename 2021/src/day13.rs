#[derive(Debug)]
enum FoldAlong {
    Row(usize),
    Col(usize),
}

struct DotMap(Vec<Vec<bool>>);

impl DotMap {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn build<'a>(input: impl Iterator<Item = &'a str>) -> DotMap {
        let (mut x_max, mut y_max) = (0, 0);
        let (xs, ys): (Vec<usize>, Vec<usize>) = input
            .filter_map(|line| line.split_once(','))
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
            .inspect(|(x, y)| {
                x_max = x_max.max(*x);
                y_max = y_max.max(*y);
            })
            .unzip();

        let mut ret = vec![vec![false; x_max + 1]; y_max + 1];
        xs.into_iter()
            .zip(ys.into_iter())
            .for_each(|(x, y)| ret[y][x] = true);
        DotMap(ret)
    }

    fn print(&self) {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                eprint!("{}", if self.0[y][x] { "●" } else { " " })
            }
            eprintln!()
        }
        eprintln!()
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter())
            .flatten()
            .filter(|dot| **dot)
            .count()
    }

    fn fold(mut self, fold_along: FoldAlong) -> Self {
        match fold_along {
            FoldAlong::Row(row) => {
                assert!(
                    row <= (self.rows() + 1) / 2,
                    "Bottom fold larger than top {} <> {}",
                    row,
                    self.0.len() / 2
                );
                let bottom = self.0.split_off(row + 1);
                self.0.pop();
                DotMap(
                    self.0
                        .into_iter()
                        .zip(bottom.into_iter().rev())
                        .map(|(top_vec, bot_vec)| {
                            top_vec
                                .into_iter()
                                .zip(bot_vec.into_iter())
                                .map(|(t, b)| t | b)
                                .collect()
                        })
                        .collect(),
                )
            }
            FoldAlong::Col(col) => {
                assert!(col <= (self.cols() + 1) / 2, "Right fold larger than left");
                DotMap(
                    self.0
                        .into_iter()
                        .map(|mut row| {
                            let right = row.split_off(col + 1);
                            row.pop();
                            row.into_iter()
                                .zip(right.into_iter().rev())
                                .map(|(l, r)| l | r)
                                .collect()
                        })
                        .collect(),
                )
            }
        }
    }
}

fn get_folds<'a>(input: impl Iterator<Item = &'a str>) -> Vec<FoldAlong> {
    input
        .map(|line| {
            let fold = line.split(' ').last().unwrap().split_once('=').unwrap();
            match fold.0 {
                "y" => FoldAlong::Row(fold.1.parse().unwrap()),
                "x" => FoldAlong::Col(fold.1.parse().unwrap()),
                _ => panic!("Invalid fold axis"),
            }
        })
        .collect()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut input = input.trim().lines().map(|line| line.trim());
    let mut dot_map = DotMap::build(input.by_ref().take_while(|line| !line.is_empty()));
    let mut folds = get_folds(input);

    dot_map = dot_map.fold(folds.remove(0));
    let d13p1 = dot_map.count();

    dot_map = folds.into_iter().fold(dot_map, |dot_map, fold_along| {
        // eprintln!("{:?} - {:?}", fold_along, dot_map.dimension());
        dot_map.fold(fold_along)
    });

    dot_map.print();

    (d13p1, dot_map.count())
}

#[test]
fn test() {
    let input = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    assert_eq!(run(input), (17, 16));
}
