pub fn run(input: &'static str) -> (usize, usize) {
    enum Command {
        Forward(usize),
        Downward(usize),
        Upward(usize),
        Invalid,
    }

    let commands = input
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

    let d2p1 = distance * depth;

    let (distance, depth, _aim) = commands
        .iter()
        .fold((0, 0, 0), |(distance, depth, aim), cmd| match cmd {
            Command::Downward(incr) => (distance, depth, aim + incr),
            Command::Upward(incr) => (distance, depth, aim - incr),
            Command::Forward(incr) => (distance + incr, depth + (incr * aim), aim),
            Command::Invalid => (distance, depth, aim),
        });

    let d2p2 = distance * depth;

    (d2p1, d2p2)
}

#[test]
fn test() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    assert_eq!(run(input), (150, 900));
}
