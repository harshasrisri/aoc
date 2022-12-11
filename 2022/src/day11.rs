use sscanf::sscanf;
use std::collections::VecDeque;

type Operation = Box<dyn Fn(u128) -> u128>;
type Test = Box<dyn Fn(u128) -> usize>;

struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,
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
            .map(|n| n.trim().parse::<u128>().unwrap())
            .collect();

        let operation = lines[2].split_once('=').unwrap().1.trim();
        let (op, num) = sscanf!(operation, "old {char} {String}").unwrap();
        let operation = match (op, num) {
            ('+', num) => {
                Box::new(move |n: u128| n + num.parse::<u128>().unwrap_or(n)) as Operation
            }
            ('*', num) => {
                Box::new(move |n: u128| n.saturating_mul(num.parse::<u128>().unwrap_or(n))) as Operation
            }
            (op, num) => panic!("Unhandled operation: old {op} {num}"),
        };

        let divisor = sscanf!(lines[3].trim(), "Test: divisible by {u128}").unwrap();
        let pass = sscanf!(lines[4].trim(), "If true: throw to monkey {usize}").unwrap();
        let fail = sscanf!(lines[5].trim(), "If false: throw to monkey {usize}").unwrap();
        let test = Box::new(move |n: u128| [fail, pass][(n % divisor == 0) as usize]);

        Monkey {
            items,
            operation,
            test,
            inspections: 0,
        }
    }

    fn throw(&mut self) -> Result<(u128, usize), ()> {
        let item_worry = self.items.pop_front().ok_or(())?;
        self.inspections += 1;
        let item_worry = (self.operation)(item_worry) / 3;
        let other = (self.test)(item_worry);
        Ok((item_worry, other))
    }

    fn catch(&mut self, item_worry: u128) {
        self.items.push_back(item_worry);
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(Monkey::new)
        // .inspect(|monkey| eprintln!("{:?}", monkey.items))
        .collect();

    for _round in 1..=20 {
        for cur in 0..monkeys.len() {
            // eprintln!("Round: {round}, Monkey: {cur}");
            while let Ok((item, other)) = monkeys[cur].throw() {
                // eprintln!("\tThrowing {item} to monkey {other}");
                monkeys[other].catch(item);
            }
        }
    }

    let (t2, t1) = monkeys
        .iter()
        .enumerate()
        // .inspect(|(i, monkey)| {
        //     eprintln!("Monkey {i}: ({}) - {:?}", monkey.inspections, monkey.items);
        // })
        .fold((0, 0), |(t2, t1), (_, monkey)| {
            match (monkey.inspections > t2, monkey.inspections > t1) {
                (true, true) => (t1, monkey.inspections),
                (true, false) => (monkey.inspections, t1),
                _ => (t2, t1),
            }
        });

    (t2 * t1, 0)
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
    assert_eq!(run(input), (10605, 0));
}
