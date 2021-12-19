#[derive(Debug)]
enum FoldAlong {
    Row(usize),
    Col(usize),
}

#[derive(Ord, PartialEq, PartialOrd, Eq)]
struct CoOrd {
    x: usize,
    y: usize,
}

struct DotMap {
    coords: Vec<CoOrd>,
    x_max: usize,
    y_max: usize,
}

impl DotMap {
    fn build<'a>(input: impl Iterator<Item = &'a str>) -> DotMap {
        let (mut x_max, mut y_max) = (0, 0);
        let coords = input
            .filter_map(|line| line.split_once(','))
            .map(|(x, y)| {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                x_max = x_max.max(x);
                y_max = y_max.max(y);
                CoOrd { x, y }
            })
            .collect();

        DotMap {
            coords,
            x_max,
            y_max,
        }
    }

    fn count(&mut self) -> usize {
        self.coords.sort();
        self.coords.dedup();
        self.coords.len()
    }

    fn fold(mut self, fold_along: FoldAlong) -> Self {
        match fold_along {
            FoldAlong::Row(y) => {
                self.coords
                    .iter_mut()
                    .filter(|coord| coord.y > y)
                    .for_each(|coord| {
                        let diff = coord.y - y;
                        coord.y -= 2 * diff;
                    });
                self.y_max = y - 1;
            }
            FoldAlong::Col(x) => {
                self.coords
                    .iter_mut()
                    .filter(|coord| coord.x > x)
                    .for_each(|coord| {
                        let diff = coord.x - x;
                        coord.x -= 2 * diff;
                    });
                self.x_max = x - 1;
            }
        }
        self
    }

    fn print(&self) {
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                eprint!(
                    "{}",
                    if self.coords.contains(&CoOrd { x, y }) {
                        '█'
                    } else {
                        ' '
                    }
                );
            }
            eprintln!();
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

    dot_map = folds
        .into_iter()
        .fold(dot_map, |dot_map, fold_along| dot_map.fold(fold_along));

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
