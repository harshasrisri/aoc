use itertools::Itertools;

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input.trim()
        .split("\n\n")
        .map(|matrix| matrix.lines().map(|line| line.chars().collect_vec()).collect_vec())
        .map(|matrix| (matrix.clone(), transpose(matrix)))
        .map(|(m, t)| (axis(&m), axis(&t)))
        .map(|(x, y)| x.unwrap_or_default() * 100 + y.unwrap_or_default())
        .sum();

    (p1, 0)
}

fn axis(matrix: &[Vec<char>]) -> Option<usize> {
    (1..matrix.len()).position(|row| (0..row).rev().zip(row..matrix.len()).all(|(up, down)| matrix[up] == matrix[down])).map(|row| row + 1)
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let ret = vec![Vec::new(); matrix[0].len()];
    matrix.into_iter().fold(ret, |mut v, row| {
        row.into_iter().enumerate().for_each(|(col, cell)| v[col].push(cell));
        v
    })
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
