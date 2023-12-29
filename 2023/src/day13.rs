pub fn run(input: &'static str) -> (usize, usize) {
    let mirrors = input.trim()
        .split("\n\n")
        .map(|matrix| matrix.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
        .map(|matrix| (matrix.clone(), transpose(matrix)))
        .collect::<Vec<_>>();

    let p1 = mirrors.iter()
        .map(|(m, t)| (axis(m), axis(t)))
        .map(|(x, y)| x.unwrap_or_default() * 100 + y.unwrap_or_default())
        .sum();

    let p2 = mirrors.iter()
        .map(|(m, t)| (axis_2(m), axis_2(t)))
        .map(|(x, y)| x.unwrap_or_default() * 100 + y.unwrap_or_default())
        .sum();

    (p1, p2)
}

fn axis(matrix: &[Vec<char>]) -> Option<usize> {
    (1..matrix.len()).position(|row| (0..row).rev().zip(row..matrix.len()).all(|(up, down)| matrix[up] == matrix[down])).map(|row| row + 1)
}

fn axis_2(matrix: &[Vec<char>]) -> Option<usize> {
    let row_diff = |up: &[char], down: &[char]| -> usize { up.iter().zip(down.iter()).filter(|(u, d)| u != d).count() };
    (1..matrix.len()).position(|row| 
            (0..row).rev().zip(row..matrix.len())
                .map(|(up, down)| row_diff(&matrix[up], &matrix[down]))
                .sum::<usize>() == 1
        )
        .map(|row| row + 1)
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
    assert_eq!(run(input), (405, 400));
}
