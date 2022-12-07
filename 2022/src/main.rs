mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn fatal(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok());
    let (p1, p2) = match arg {
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
