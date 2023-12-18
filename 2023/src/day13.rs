use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    axis: usize,
    depth: usize,
}

#[derive(Debug)]
enum Reflected {
    Horizontal(Grid),
    Vertical(Grid),
}

fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i].clone()).collect::<Vec<_>>())
        .collect()
}

impl Reflected {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let horz_axis = grid.windows(2).find_position(|w| w[0] == w[1]).map(|(n, _)| n + 1);
        let horz_depth = if let Some(n) = horz_axis {
            (0..n).rev().zip(n..grid.len()).take_while(|(up, down)| grid[*up] == grid[*down]).count()
        } else {
            0
        };

        let t_grid = transpose(grid.clone());
        let vert_axis = t_grid.windows(2).find_position(|w| w[0] == w[1]).map(|(n, _)| n + 1);
        let vert_depth = if let Some(n) = vert_axis {
            (0..n).rev().zip(n..t_grid.len()).take_while(|(left, right)| t_grid[*left] == t_grid[*right]).count()
        } else {
            0
        };

        if horz_depth > vert_depth {
            Self::Horizontal(Grid { grid, axis: horz_axis.unwrap(), depth: horz_depth })
        } else {
            Self::Vertical(Grid { grid: t_grid, axis: vert_axis.unwrap(), depth: vert_depth })
        }
    }

    fn get_score(&self) -> usize {
        match self {
            Self::Horizontal(grid) => 100 * grid.axis,
            Self::Vertical(grid) => grid.axis,
        }
    }

    fn inner(&self) -> &Grid {
        match self {
            Self::Horizontal(g) => g,
            Self::Vertical(g) => g,
        }
    }

}

pub fn run(input: &'static str) -> (usize, usize) {
    let reflections = input.split("\n\n")
        // .inspect(|line| println!("{line}"))
        .map(Reflected::from)
        // .inspect(|r| println!("Axis: {}\n", r.inner().axis))
        .collect_vec();
    let p1 = reflections.iter().map(|r| r.get_score()).sum();
    (p1, 0)
}

#[test]
fn test1() {
    let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!(run(input), (405, 0));
}
