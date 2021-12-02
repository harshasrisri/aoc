use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn day1() {
    let file = File::open("inputs/d01.txt").unwrap();
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let d1p1 = data.windows(2).filter(|w| w[0] < w[1]).count();
    println!("Day 01, Part 1: {}", d1p1);

    let d1p2 = data
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();
    println!("Day 01, Part 2: {}", d1p2);
}

fn day2() {
    let file = File::open("inputs/d02.txt").unwrap();
    let reader = BufReader::new(file);

    enum Command {
        Forward(usize),
        Downward(usize),
        Upward(usize),
        Invalid,
    }

    let movements = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            if let Some((dir, steps)) = line.split_once(' ') {
                let steps = steps.parse::<usize>().unwrap();
                match dir {
                    "forward" => Command::Forward(steps),
                    "down" => Command::Downward(steps),
                    "up" => Command::Upward(steps),
                    _ => Command::Invalid,
                }
            } else {
                Command::Invalid
            }
        })
        .collect::<Vec<_>>();

    let (distance, depth) = movements
        .iter()
        .fold((0, 0), |(distance, depth), cmd| match cmd {
            Command::Downward(n) => (distance, depth + n),
            Command::Upward(n) => (distance, depth - n),
            Command::Forward(n) => (distance + n, depth),
            Command::Invalid => (distance, depth),
        });

    println!("Day 02, Part 1: {}, {}, {}", distance, depth, distance * depth);

    let mut aim = 0;
    let (distance, depth) = movements
        .iter()
        .fold((0, 0), |(distance, depth), cmd| match cmd {
            Command::Downward(n) => { aim += n; (distance, depth) },
            Command::Upward(n) => { aim -= n; (distance, depth) },
            Command::Forward(n) => (distance + n, depth + (n * aim)),
            Command::Invalid => (distance, depth),
        });

    println!("Day 02, Part 2: {}, {}, {}", distance, depth, distance * depth);
}

fn main() {
    day1();
    day2();
}
