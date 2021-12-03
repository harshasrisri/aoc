fn day1() {
    let data = include_str!("../inputs/d01.txt")
        .lines()
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
    enum Command {
        Forward(usize),
        Downward(usize),
        Upward(usize),
        Invalid,
    }

    let commands = include_str!("../inputs/d02.txt")
        .lines()
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

    let (distance, depth) = commands
        .iter()
        .fold((0, 0), |(distance, depth), cmd| match cmd {
            Command::Downward(incr) => (distance, depth + incr),
            Command::Upward(incr) => (distance, depth - incr),
            Command::Forward(incr) => (distance + incr, depth),
            Command::Invalid => (distance, depth),
        });

    println!("Day 02, Part 1: {}, {}, {}", distance, depth, distance * depth);

    let (distance, depth, _aim) = commands
        .iter()
        .fold((0, 0, 0), |(distance, depth, aim), cmd| match cmd {
            Command::Downward(incr) => (distance, depth, aim + incr),
            Command::Upward(incr) => (distance, depth, aim - incr),
            Command::Forward(incr) => (distance + incr, depth + (incr * aim), aim),
            Command::Invalid => (distance, depth, aim),
        });

    println!("Day 02, Part 2: {}, {}, {}", distance, depth, distance * depth);
}

fn main() {
    day1();
    day2();
}
