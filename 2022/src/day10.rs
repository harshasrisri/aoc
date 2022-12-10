#[derive(Debug, Clone)]
enum Op {
    Noop,
    Addx(isize),
}

pub fn run(input: &'static str) -> (usize, usize) {
    let op_iter = input
        .lines()
        .flat_map(|line| {
            if line == "noop" {
                vec![Op::Noop]
            } else {
                let op = Op::Addx(line.split_once(' ').unwrap().1.parse().unwrap());
                vec![Op::Noop, op]
            }
        });

    let mut cumulation = 1;
    let sums = std::iter::repeat(Op::Noop).take(1).chain(op_iter)
        .zip(1..)
        .map(|(op, cycle)| {
            cumulation += match op {
                Op::Noop => 0,
                Op::Addx(x) => x,
            };
            eprintln!("{cycle}, {cumulation}, {:?}", op);
            cumulation * cycle as isize
        })
        .collect::<Vec<_>>();

    let p1 = sums
        .into_iter()
        .skip(19)
        .step_by(40)
        .inspect(|signal| eprintln!("signal: {signal}"))
        .sum::<isize>();

    (p1 as usize, 0)
}

#[test]
fn test() {
    let input = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    assert_eq!(run(input), (13140,0));
}
