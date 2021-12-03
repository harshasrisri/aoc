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

    println!(
        "Day 02, Part 1: {}, {}, {}",
        distance,
        depth,
        distance * depth
    );

    let (distance, depth, _aim) = commands
        .iter()
        .fold((0, 0, 0), |(distance, depth, aim), cmd| match cmd {
            Command::Downward(incr) => (distance, depth, aim + incr),
            Command::Upward(incr) => (distance, depth, aim - incr),
            Command::Forward(incr) => (distance + incr, depth + (incr * aim), aim),
            Command::Invalid => (distance, depth, aim),
        });

    println!(
        "Day 02, Part 2: {}, {}, {}",
        distance,
        depth,
        distance * depth
    );
}

fn day3() {
    let signals = include_str!("../inputs/d03.txt")
        .lines()
        .map(|bits| {
            bits.chars()
                .map(|bit| match bit {
                    '0' => -1,
                    '1' => 1,
                    _ => panic!("invalid bit {}", bit),
                })
                .collect::<Vec<isize>>()
        })
        .fold(Vec::new(), |acc: Vec<isize>, signals| {
            let acc = if acc.is_empty() {
                vec![0; signals.len()]
            } else {
                acc
            };
            acc.iter().zip(signals.iter()).map(|(a, b)| a + b).collect()
        });

    let (gamma, epsilon) =
        signals
            .iter()
            .fold((0, 0), |(gamma, epsilon), signal| match signal.cmp(&0) {
                std::cmp::Ordering::Greater => ((gamma << 1) | 1, epsilon << 1),
                std::cmp::Ordering::Less => (gamma << 1, (epsilon << 1) | 1),
                std::cmp::Ordering::Equal => panic!("Not expecting 0 here"),
            });

    println!(
        "Day 03, Part 1: {}, {}, {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn main() {
    let arg = std::env::args().skip(1).next().map(|arg| arg.parse::<isize>().ok()).flatten();
    match arg {
        Some(3) => day3(),
        Some(2) => day2(),
        Some(1) => day1(),
        Some(arg) => eprintln!("{} - Not implemented", arg),
        None => eprintln!("Invalid argument"),
    }
}
