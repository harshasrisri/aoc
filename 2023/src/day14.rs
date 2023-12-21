use std::collections::{HashMap, BTreeMap};
use itertools::Itertools;

fn tilt_north(grid: &mut[Vec<char>]) {
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            let mut r = row;
            while r < grid.len() && grid[r][col] == '.' {
                r += 1;
            }
            if r < grid.len() && grid[r][col] == 'O' {
                grid[r][col] = '.';
                grid[row][col] = 'O';
            }
        }
    }
}

fn tilt_south(grid: &mut[Vec<char>]) {
    for col in 0..grid[0].len() {
        for row in (0..grid.len()).rev() {
            let mut r = row;
            while grid[r][col] == '.' {
                if r == 0 { break; }
                r -= 1;
            }
            if grid[r][col] == 'O' {
                grid[r][col] = '.';
                grid[row][col] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut[Vec<char>]) {
    grid.iter_mut().for_each(|row| {
        row.split_mut(|c| *c == '#').for_each(|slice| slice.sort_by(|a, b| b.cmp(a)));
    });
}

fn tilt_east(grid: &mut[Vec<char>]) {
    grid.iter_mut().for_each(|row| {
        row.split_mut(|c| *c == '#').for_each(|slice| slice.sort());
    })
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut cycle_map = HashMap::new();

    let mut rocks = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    cycle_map.insert(rocks.clone(), 0);

    tilt_north(&mut rocks);
    let calc_load = |rocks: &Vec<Vec<char>>| -> usize {
        rocks.iter().enumerate().map(|(row, rock_line)| {
            rock_line.iter().filter(|r| **r == 'O').count() * (rocks.len() - row)
        }).sum()
    };

    let p1 = calc_load(&rocks);

    tilt_west(&mut rocks);
    tilt_south(&mut rocks);
    tilt_east(&mut rocks);

    let mut cycle_end = 1;
    cycle_map.insert(rocks.clone(), cycle_end);

    let cycle_start = loop {
        tilt_north(&mut rocks);
        tilt_west(&mut rocks);
        tilt_south(&mut rocks);
        tilt_east(&mut rocks);
        cycle_end += 1;
        if let Some(prev) = cycle_map.insert(rocks.clone(), cycle_end) {
            println!("Found cycle from {prev} to {cycle_end} cycles");
            cycle_map.insert(rocks, prev);
            break prev;
        }
    };

    const BILLION: usize = 1_000_000_000;
    let cycle_len = cycle_end - cycle_start;
    let cycle_count = (BILLION - cycle_start) / cycle_len;
    let remainder = BILLION - (cycle_start + cycle_count * cycle_len);

    let cycle_map = cycle_map.into_iter().map(|(key, value)| (value, key)).collect::<BTreeMap<_, _>>();
    let rocks = cycle_map.get(&(cycle_start + remainder)).unwrap();

    let p2 = calc_load(rocks);

    (p1, p2)
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
    assert_eq!(run(input), (136, 64));
}
