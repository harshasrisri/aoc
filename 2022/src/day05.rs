use sscanf::sscanf;

fn transpose<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i].clone()).collect::<Vec<_>>())
        .collect()
}

struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

pub fn run(input: &'static str) -> (String, String) {
    let stacks = input
        .lines()
        .take_while(|line| line.contains('['))
        .map(|line| {
            let vline = line
                .chars()
                .chain(std::iter::repeat(' '))
                .take(9 * 4)
                .collect::<Vec<_>>();
            vline.chunks(4).map(|chunk| chunk[1]).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let skip_lines = stacks.len() + 2; // stack spec + stack num + blank
    let operations = input
        .lines()
        .skip(skip_lines)
        .map(|line| {
            let (count, from, to) =
                sscanf!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
            Operation {
                count,
                from: from - 1,
                to: to - 1,
            } // 1-index to 0-index for Vec
        })
        .collect::<Vec<_>>();

    // transpose and remove empty slots
    let mut stacks = transpose(stacks)
        .into_iter()
        .map(|stack| {
            stack
                .into_iter()
                .filter(|c| *c != ' ')
                .rev()
                .collect::<Vec<_>>()
        })
        .filter(|stack| !stack.is_empty())
        .collect::<Vec<_>>();

    let mut p1_stacks = stacks.clone();
    for op in operations.iter() {
        for _ in 0..op.count {
            let mov = p1_stacks[op.from].pop().unwrap();
            p1_stacks[op.to].push(mov);
        }
    }

    let p1 = p1_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    for op in operations {
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
