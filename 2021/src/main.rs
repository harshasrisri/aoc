mod day1;
mod day2;
mod day3;

fn fatal(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .map(|arg| arg.parse::<usize>().ok())
        .flatten();
    let (p1, p2) = match arg {
        Some(3) => day3::run(include_str!("../inputs/d03.txt")),
        Some(2) => day2::run(include_str!("../inputs/d02.txt")),
        Some(1) => day1::run(include_str!("../inputs/d01.txt")),
        Some(arg) => fatal(format!("Day {} - Not implemented", arg).as_str()),
        None => fatal("Invalid argument. Pass a number corresponding to the day."),
    };
    println!("Day {}: Part1: {}, Part2: {}", arg.unwrap(), p1, p2);
}
