use itertools::Itertools;
use std::cmp::Ordering;
use array2d::Array2D;

pub fn run(input: &'static str) -> (usize, usize) {
    let (rows, cols) = (input.lines().count(), input.lines().next().unwrap().len());
    let tree_iter = input.chars().filter_map(|c| c.to_digit(10).map(|n| n as usize));
    let forest = Array2D::from_iter_row_major(tree_iter, rows, cols).unwrap();

    let mut num_visible = 2 * (rows + cols) - 4;
    let mut max_score = 0;

    let rows_iter = forest.rows_iter().map(|row| row.collect::<Vec<_>>()).enumerate().skip(1).take(forest.num_rows() - 2);
    let cols_iter = forest.columns_iter().map(|col| col.collect::<Vec<_>>()).enumerate().skip(1).take(forest.num_columns() - 2);

    for ((x, row), (y, col)) in rows_iter.cartesian_product(cols_iter) {
        // Iterators for left, right, up and down from a particular tree till the edge
        let lt = row.get(0..y).unwrap().iter().rev();
        let rt = row.get(y + 1..).unwrap().iter();
        let up = col.get(0..x).unwrap().iter().rev();
        let dn = col.get(x + 1..).unwrap().iter();

        let tree_height = &row[y];
        num_visible += (lt.clone().max().unwrap() < tree_height
            || rt.clone().max().unwrap() < tree_height
            || up.clone().max().unwrap() < tree_height
            || dn.clone().max().unwrap() < tree_height) as usize;

        let height_score = |skip, count, height: usize| -> (bool, usize) {
            match (skip, height.cmp(tree_height)) {
                (false, Ordering::Less) => (false, count + 1),
                (false, Ordering::Equal) => (true, count + 1),
                (false, Ordering::Greater) => (true, count + 1),
                _ => (true, count),
            }
        };

        let mut score = 1;
        score *= lt.fold((false, 0), |(skip, count), height| height_score(skip, count, **height) ).1;
        score *= rt.fold((false, 0), |(skip, count), height| height_score(skip, count, **height) ).1;
        score *= up.fold((false, 0), |(skip, count), height| height_score(skip, count, **height) ).1;
        score *= dn.fold((false, 0), |(skip, count), height| height_score(skip, count, **height) ).1;

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
