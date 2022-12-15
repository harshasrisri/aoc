mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
#[rustfmt::skip]
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn fatal(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok());
    let (p1, p2) = match arg {
        Some(15) => day15::run(include_str!("../inputs/day15.txt")),
        Some(14) => day14::run(include_str!("../inputs/day14.txt")),
        Some(13) => day13::run(include_str!("../inputs/day13.txt")),
        Some(12) => day12::run(include_str!("../inputs/day12.txt")),
        Some(11) => day11::run(include_str!("../inputs/day11.txt")),
        Some(10) => day10::run(include_str!("../inputs/day10.txt")),
        Some(9) => day09::run(include_str!("../inputs/day09.txt")),
        Some(8) => day08::run(include_str!("../inputs/day08.txt")),
        Some(7) => day07::run(include_str!("../inputs/day07.txt")),
        Some(6) => day06::run(include_str!("../inputs/day06.txt")),
        Some(5) => {
            let (p1, p2) = day05::run(include_str!("../inputs/day05.txt"));
            println!("Day {}: Part1: {}, Part2: {}", arg.unwrap(), p1, p2);
            return;
        }
        Some(4) => day04::run(include_str!("../inputs/day04.txt")),
        Some(3) => day03::run(include_str!("../inputs/day03.txt")),
        Some(2) => day02::run(include_str!("../inputs/day02.txt")),
        Some(1) => day01::run(include_str!("../inputs/day01.txt")),
        Some(arg) => fatal(format!("Day {} - Not implemented", arg).as_str()),
        None => fatal("Invalid argument. Pass a number corresponding to the day."),
    };
    println!("Day {}: Part1: {}, Part2: {}", arg.unwrap(), p1, p2);
}
