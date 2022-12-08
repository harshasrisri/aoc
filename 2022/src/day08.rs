use std::cmp::Ordering;

use itertools::Itertools;

fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i].clone()).collect::<Vec<_>>())
        .collect()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let tree_rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or_default() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let tree_cols = transpose(tree_rows.clone());
    let mut visible = tree_rows.len() * 2 + tree_cols.len() * 2 - 4;
    let mut max_score = 0;

    for ((x, row), (y, col)) in tree_rows
        .iter()
        .enumerate()
        .cartesian_product(tree_cols.iter().enumerate())
    {
        if x == 0 || y == 0 || x == row.len() - 1 || y == col.len() - 1 {
            continue;
        }

        assert_eq!(row[y], col[x]);

        let tree_height = row[y];
        let lt = *row.get(0..y).unwrap().iter().max().unwrap() < tree_height;
        let rt = *row.get(y + 1..).unwrap().iter().max().unwrap() < tree_height;
        let up = *col.get(0..x).unwrap().iter().max().unwrap() < tree_height;
        let dn = *col.get(x + 1..).unwrap().iter().max().unwrap() < tree_height;
        if lt || rt || up || dn {
            visible += 1;
        }
        // eprintln!("[{},{}] == {} == {} ({})", x, y, row[y], col[x], visible);

        let height_score = |skip, count, height: usize| -> (bool, usize) {
            match (skip, height.cmp(&tree_height)) {
                (false, Ordering::Less) => (false, count + 1),
                (false, Ordering::Equal) => (true, count + 1),
                (false, Ordering::Greater) => (true, count + 1),
                (_, _) => (true, count),
            }
        };

        let lt = row
            .get(0..y)
            .unwrap()
            .iter()
            .rev()
            .fold((false, 0), |(skip, count), height| {
                height_score(skip, count, *height)
            })
            .1;
        let rt = row
            .get(y+1..)
            .unwrap()
            .iter()
            .fold((false, 0), |(skip, count), height| {
                height_score(skip, count, *height)
            })
            .1;
        let up = col
            .get(0..x)
            .unwrap()
            .iter()
            .rev()
            .fold((false, 0), |(skip, count), height| {
                height_score(skip, count, *height)
            })
            .1;
        let dn = col
            .get(x+1..)
            .unwrap()
            .iter()
            .fold((false, 0), |(skip, count), height| {
                height_score(skip, count, *height)
            })
            .1;

        max_score = max_score.max(lt * rt * up * dn);
        eprintln!( "[{x},{y}] == {} ({lt}, {rt}, {up}, {dn}, {max_score})", row[y]);
    }

    (visible, max_score)
}

#[test]
fn test() {
    let input = "\
30373
25512
65332
33549
35390
";
    assert_eq!(run(input), (21, 8));
}
