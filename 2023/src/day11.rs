use itertools::Itertools;

pub fn run(input: &'static str) -> (usize, usize) {
    let coords = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
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
        .flatten()
        .collect::<Vec<_>>();

    let (occupied_rows, mut occupied_cols): (Vec<usize>, Vec<usize>) = coords.iter().cloned().unzip();
    occupied_cols.sort();
    let missing_rows = occupied_rows.windows(2).flat_map(|v| (v[0]+1..v[1])).collect::<Vec<_>>();
    let missing_cols = occupied_cols.windows(2).flat_map(|v| (v[0]+1..v[1])).collect::<Vec<_>>();

    let coords = coords.into_iter()
        .map(|(row, col)| {
            let row = match missing_rows.binary_search(&row) {
                Ok(_) => row,
                Err(pos) => row + pos,
            };
            let col = match missing_cols.binary_search(&col) {
                Ok(_) => col,
                Err(pos) => col + pos,
            };
            (row, col)
        })
        .collect::<Vec<_>>();

    let p1 = coords.into_iter().tuple_combinations().map(|(p1, p2)| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)).sum();

    (p1, 0)
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
    assert_eq!(run(input), (374, 0));
}
