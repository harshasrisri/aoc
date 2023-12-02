mod day01;

fn fatal(s: &str) -> ! {
    eprintln!("{}", s);
    std::process::exit(1);
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok());
    let (p1, p2) = match arg {
        Some(1) => day01::run(include_str!("../inputs/day01.txt")),
        Some(arg) => fatal(format!("Day {} - Not implemented", arg).as_str()),
        None => fatal("Invalid argument. Pass a number corresponding to the day."),
    };
    println!("Day {}: Part1: {}, Part2: {}", arg.unwrap(), p1, p2);
}
