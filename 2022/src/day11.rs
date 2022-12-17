use sscanf::sscanf;
use std::collections::VecDeque;

type Test = Box<dyn Fn(usize) -> usize>;

enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        let input = input.split_once('=').unwrap().1.trim();
        let (op, num) = sscanf!(input, "old {char} {str}").unwrap();
        match (op, num) {
            ('+', "old") => Operation::Mul(2),
            ('*', "old") => Operation::Square,
            ('+', num) => Operation::Add(num.parse().unwrap()),
            ('*', num) => Operation::Mul(num.parse().unwrap()),
            (op, num) => panic!("Unhandled operation: old {op} {num}"),
        }
    }

    fn operate(&self, input: usize) -> usize {
        match self {
            Operation::Add(n) => input.saturating_add(*n),
            Operation::Mul(n) => input.saturating_mul(*n),
            Operation::Square => input.saturating_mul(input),
        }
    }
}

struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    divisor: usize,
    inspections: usize,
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let lines: Vec<_> = input.lines().collect();
        let items: VecDeque<_> = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect();

        let operation = Operation::from_str(lines[2]);
        let divisor = sscanf!(lines[3].trim(), "Test: divisible by {usize}").unwrap();
        let pass = sscanf!(lines[4].trim(), "If true: throw to monkey {usize}").unwrap();
        let fail = sscanf!(lines[5].trim(), "If false: throw to monkey {usize}").unwrap();
        let test = Box::new(move |n: usize| [fail, pass][(n % divisor == 0) as usize]);

        Monkey {
            items,
            operation,
            test,
            inspections: 0,
            divisor,
        }
    }

    fn throw(&mut self, relief: bool) -> Result<(usize, usize), ()> {
        let old_worry = self.items.pop_front().ok_or(())?;
        let relief = 3_usize.pow(relief as u32);
        self.inspections += 1;
        let new_worry = self.operation.operate(old_worry) / relief;
        let other = (self.test)(new_worry);
        Ok((new_worry, other))
    }

    fn catch(&mut self, item_worry: usize) {
        self.items.push_back(item_worry);
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut test_prod = 1;
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(Monkey::new)
        .inspect(|monkey| test_prod *= monkey.divisor)
        .collect();

    for _round in 1..=20 {
        for cur in 0..monkeys.len() {
            while let Ok((item, other)) = monkeys[cur].throw(true) {
                monkeys[other].catch(item % test_prod);
            }
        }
    }

    let (t2, t1) = monkeys
        .iter()
        .enumerate()
        .fold((0, 0), |(t2, t1), (_, monkey)| {
            match (monkey.inspections > t2, monkey.inspections > t1) {
                (true, true) => (t1, monkey.inspections),
                (true, false) => (monkey.inspections, t1),
                _ => (t2, t1),
            }
        });

    eprintln!("p1: {t1}, {t2}");

    let p1 = t2 * t1;

    for _round in 1..=10000 {
        for cur in 0..monkeys.len() {
            while let Ok((item, other)) = monkeys[cur].throw(false) {
                monkeys[other].catch(item % test_prod);
            }
        }
    }

    let (t2, t1) = monkeys
        .iter()
        .enumerate()
        .fold((0, 0), |(t2, t1), (_, monkey)| {
            match (monkey.inspections > t2, monkey.inspections > t1) {
                (true, true) => (t1, monkey.inspections),
                (true, false) => (monkey.inspections, t1),
                _ => (t2, t1),
            }
        });

    eprintln!("p2: {t1}, {t2}");

    (p1, t2 * t1)
}

#[test]
fn test() {
    let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    assert_eq!(run(input), (10605, 2713310158));
}
