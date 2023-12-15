use itertools::Itertools;

pub fn run(input: &'static str) -> (usize, usize) {
    let coords = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if c == '#' {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let (mut occupied_rows, mut occupied_cols): (Vec<usize>, Vec<usize>) = coords.iter().cloned().unzip();
    occupied_rows.dedup();
    occupied_cols.sort();
    occupied_cols.dedup();
    let missing_rows = occupied_rows.windows(2).flat_map(|v| (v[0]+1..v[1])).collect::<Vec<_>>();
    let missing_cols = occupied_cols.windows(2).flat_map(|v| (v[0]+1..v[1])).collect::<Vec<_>>();

    let expand = |coords: &Vec<(usize, usize)>, expansion_factor| -> Vec<(usize, usize)> {
        coords.iter()
            .map(|(row, col)| {
                let row = match missing_rows.binary_search(row) {
                    Ok(_) => *row,
                    Err(pos) => *row + pos * expansion_factor,
                };
                let col = match missing_cols.binary_search(col) {
                    Ok(_) => *col,
                    Err(pos) => *col + pos * expansion_factor,
                };
                (row, col)
            })
            .collect::<Vec<_>>()
    };

    let dist_eval = |coords: &Vec<(usize, usize)>| -> usize {
        coords.iter().tuple_combinations().map(|(p1, p2)| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)).sum()
    };

    let expansion_factor = std::env::var("D11P2_TEST_RUNS").map(|v| v.parse::<usize>().ok()).ok().flatten().unwrap_or(999_999_usize);

    (
        dist_eval(&expand(&coords, 1)), 
        dist_eval(&expand(&coords, expansion_factor)), 
    )
}

#[test]
fn test1() {
    let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    std::env::set_var("D11P2_TEST_RUNS", "99");
    assert_eq!(run(input), (374, 8410));
    std::env::set_var("D11P2_TEST_RUNS", "9");
    assert_eq!(run(input), (374, 1030));
}
