use itertools::Itertools;
use std::cmp::Ordering;

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

    let mut num_visible = tree_rows.len() * 2 + tree_cols.len() * 2 - 4;
    let mut max_score = 0;

    for ((x, row), (y, col)) in tree_rows
        .iter()
        .enumerate()
        .skip(1)
        .take(tree_rows.len() - 2)
        .cartesian_product(
            tree_cols
                .iter()
                .enumerate()
                .skip(1)
                .take(tree_cols.len() - 2),
        )
    {
        // Iterators for left, right, up and down from a particular tree till the edge
        let lt = row.get(0..y).unwrap().iter().rev();
        let rt = row.get(y + 1..).unwrap().iter();
        let up = col.get(0..x).unwrap().iter().rev();
        let dn = col.get(x + 1..).unwrap().iter();

        let tree_height = row[y];
        let mut visible = false;
        visible |= lt.clone().max().unwrap() < &tree_height;
        visible |= rt.clone().max().unwrap() < &tree_height;
        visible |= up.clone().max().unwrap() < &tree_height;
        visible |= dn.clone().max().unwrap() < &tree_height;
        num_visible += visible as usize;

        let height_score = |skip, count, height: usize| -> (bool, usize) {
            match (skip, height.cmp(&tree_height)) {
                (false, Ordering::Less) => (false, count + 1),
                (false, Ordering::Equal) => (true, count + 1),
                (false, Ordering::Greater) => (true, count + 1),
                (_, _) => (true, count),
            }
        };

        let mut score = 1;
        score *= lt.fold((false, 0), |(skip, count), height| height_score(skip, count, *height) ).1;
        score *= rt.fold((false, 0), |(skip, count), height| height_score(skip, count, *height) ).1;
        score *= up.fold((false, 0), |(skip, count), height| height_score(skip, count, *height) ).1;
        score *= dn.fold((false, 0), |(skip, count), height| height_score(skip, count, *height) ).1;

        max_score = max_score.max(score);
    }

    (num_visible, max_score)
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
