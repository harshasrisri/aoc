use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
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
