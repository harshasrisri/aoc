use sscanf::sscanf;

fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>> 
where 
    T: Clone,
{
    (0..input[0].len()) .map(|i| {
            input.iter()
                .map(|row| row[i].clone())
                .collect::<Vec<_>>()
        })
        .collect()
}

struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

pub fn run(input: &'static str) -> (String, String) {
    let mut stacks = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|row| row.replace("    ", "[.] "))
        .map(|row| row.replace("[", "").replace("]", ""))
        .map(|row| row
             .split(' ')
             .chain(std::iter::repeat("."))
             .take(9)
             .map(|s| s.to_owned())
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let skip_lines = stacks.len() + 1;
    stacks.pop(); // pop last row which has stack numbers

    // transpose and remove empty slots
    let mut stacks = transpose(stacks)
        .iter_mut()
        .map(|stack| stack.join("").replace(".", "").chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operations = input
        .lines()
        .skip(skip_lines)
        .map(|line| {
            let (count, from, to) = sscanf!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
            Operation {count, from: from - 1, to: to - 1} // 1-index to 0-index for Vec
        })
        .collect::<Vec<_>>();

    stacks.iter().for_each(|stack| eprintln!("{:?}", stack));
    let mut p1_stack = stacks.clone();
    for op in operations.iter() {
        // eprintln!("move {} from {} to {}", op.count, op.from + 1, op.to + 1);
        // stacks.iter().for_each(|stack| eprintln!("{:?}", stack));
        for _ in 0..op.count {
            let mov = p1_stack[op.from].pop().unwrap();
            p1_stack[op.to].push(mov);
        }
    }

    let p1 = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    for op in operations {
        eprintln!("move {} from {} to {}", op.count, op.from + 1, op.to + 1);
        stacks.iter().for_each(|stack| eprintln!("{:?}", stack));
        let from = &mut stacks[op.from];
        let mut tail = from.split_off(from.len() - op.count);
        stacks[op.to].append(&mut tail);
    }

    let p2 = stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect::<String>();

    (p1, p2)
}

#[test]
fn test() {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    assert_eq!(run(input), ("CMZ".to_owned(), "MCD".to_owned()));
}
