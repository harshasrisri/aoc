use itertools::Itertools;

fn traverse(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<usize> {
    if grid[row][col] == 'E' {
        return Some(1);
    }

    let (rm, cm) = (grid.len() as isize, grid[0].len() as isize);
    let mut min_dist = None;

    let neighbours = [-1, 0, 1]
        .into_iter()
        .cartesian_product([-1, 0, 1].into_iter())
        .filter(|(r, c)| (r, c) != (&0, &0))
        .map(|(r, c)| (row as isize + r, col as isize + c))
        .filter(|(r, c)| r >= &0 || c >= &0 || r < &rm || c < &cm)
        .inspect(|(r, c)| eprintln!("{r},{c}"))
        .map(|(r, c)| (r as usize, c as usize))
        .collect::<Vec<_>>();

    for (r, c) in neighbours.into_iter() {
        eprintln!("{row},{col} - neighbour: {r},{c}");
        if grid[r][c] as usize - grid[row][col] as usize != 1 {
            continue;
        }
        let dist = traverse(grid, r, c);
        min_dist = match (min_dist, dist) {
            (m, None) => m,
            (None, d) => d,
            (Some(m), Some(d)) => Some(m.min(d)),
        }
    }
    min_dist
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (mut r, mut c) = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| line
            .trim()
            .chars()
            .enumerate()
            .map(|(col, ch)| {
                if ch == 'S' { 
                    (r,c) = (row, col); 
                }
                ch
            })
            .collect()
        )
        .collect();

    let p1 = traverse(&grid, r, c).unwrap();

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
    assert_eq!(run(input), (31,0));
}
