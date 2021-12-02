use std::{fs::File, io::{BufRead, BufReader}};

fn day1() {
    let file = File::open("inputs/d01.txt").unwrap();
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let d1p1 = data
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    let d1p2 = data
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();


    println!("{}, {}", d1p1, d1p2);
}

fn day2() {
    let file = File::open("inputs/d02.txt").unwrap();
    let reader = BufReader::new(file);

    let movements = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            if let Some((dir, steps)) = line.split_once(' ') {
                let steps = steps.parse::<isize>().unwrap();
                match dir {
                    "forward" => (steps, 0),
                    "down" => (0, steps),
                    "up" => (0, -1 * steps),
                    _ => (0, 0),
                }
            } else { (0,0) }
        })
    .collect::<Vec<(isize, isize)>>();

    let distance: isize = movements.iter().map(|t| t.0).sum();
    let depth: isize  = movements.iter().map(|t| t.1).sum();

    println!("{}, {}, {}", distance, depth, distance * depth);
}

fn main() {
    day1();
    day2();
}
