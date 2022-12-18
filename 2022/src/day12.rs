fn traverse(grid: &Vec<Vec<isize>>, row: usize, col: usize) -> Option<usize> {
    if grid[row][col] == 27 {
        return Some(1);
    }

    let (r, c) = (row as isize, col as isize);
    let (rm, cm) = (grid.len() as isize, grid[0].len() as isize);
    let neighbours = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        .into_iter()
        .filter_map(|(r, c)| if r >= 0 && r < rm && c >= 0 && c < cm {
            Some((r as usize, c as usize))
        } else {
            None
        })
        .collect::<Vec<_>>();

    let mut min_dist: Option<usize> = None;
    for (r, c) in neighbours {
        if ![0, 1].contains(&(grid[r][c] - grid[row][col])) {
            continue;
        }
        eprintln!("Cell[{row}][{col}] ({}) - neighbour[{r}][{c}] ({})", grid[row][col], grid[r][c]);
        min_dist = match (min_dist, traverse(grid, r, c)) {
            (Some(m), Some(d)) => Some(m.min(d)),
            (Some(m), None) => Some(m),
            (None, Some(d)) => Some(d),
            (None, None) => None,
        }
    }

    min_dist
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (mut r, mut c) = (0, 0);
    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    match ch {
                        'S' => {
                            (r, c) = (row, col);
                            -1
                        }
                        'E' => 26,
                        ch => ch as isize - 'a' as isize,
                    }
                })
                .collect()
        })
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
    assert_eq!(run(input), (31, 0));
}
