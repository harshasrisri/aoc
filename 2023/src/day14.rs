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
    let rocks  = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut t_rocks = transpose(rocks);

    for row in &mut t_rocks {
        row.split_mut(|c| *c == '#').for_each(|slice| slice.sort_by(|a, b| b.cmp(&a)));
    }

    let rocks = transpose(t_rocks);
    let p1 = rocks.iter().enumerate().map(|(row, rock_line)| {
        rock_line.iter().filter(|r| **r == 'O').count() * (rocks.len() - row)
    }).sum();

    (p1, 0)
}

#[test]
fn test1() {
    let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    assert_eq!(run(input), (136, 0));
}
