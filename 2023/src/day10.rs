use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

static RUBRIC: LazyLock<HashMap<char, HashMap<Heading, Heading>>> = LazyLock::new(|| {
    let mut rubric: HashMap<char, HashMap<Heading, Heading>> = HashMap::new();
    let pipe_headings: [(char, (Heading, Heading)); 12] = [
        ('|', (Heading::North, Heading::North)),
        ('|', (Heading::South, Heading::South)),
        ('-', (Heading::East, Heading::East)),
        ('-', (Heading::West, Heading::West)),
        ('L', (Heading::South, Heading::East)),
        ('L', (Heading::West, Heading::North)),
        ('J', (Heading::South, Heading::West)),
        ('J', (Heading::East, Heading::North)),
        ('7', (Heading::East, Heading::South)),
        ('7', (Heading::North, Heading::West)),
        ('F', (Heading::North, Heading::East)),
        ('F', (Heading::West, Heading::South)),
    ];

    for (pipe, (incoming, outgoing)) in pipe_headings {
        rubric.entry(pipe)
            .and_modify(|mp| { mp.insert(incoming, outgoing); })
            .or_insert(HashMap::from([(incoming, outgoing)]));
    }
    rubric
});

impl Heading {
    fn next(&self, pipe: char) -> Option<Self> {
        RUBRIC.get(&pipe)?.get(self).copied()
    }

    fn check(&self, pipe: char) -> Option<()> {
        if RUBRIC.get(&pipe)?.contains_key(self) {
            Some(())
        } else {
            None
        }
    }
}

fn walk(input: &[Vec<char>], row: usize, col: usize, heading: Heading) -> Option<usize> {
    let cur_cell = input.get(row).map(|rw| rw.get(col)).flatten()?;
    if *cur_cell == 'S' {
        return Some(1);
    }
    let _ = heading.check(*cur_cell)?;
    let next_heading = heading.next(*cur_cell)?;
    let next_cell = match next_heading {
        Heading::South => (row + 1, col),
        Heading::North => (row.checked_sub(1)?, col),
        Heading::East => (row, col + 1),
        Heading::West => (row, col.checked_sub(1)?),
    };
    walk(input, next_cell.0, next_cell.1, next_heading).map(|dist| dist + 1)
}

fn begin(input: &[Vec<char>], row: usize, col: usize) -> usize {
    walk(input, row+1, col, Heading::South)
        .or_else(|| walk(input, row, col+1, Heading::East))
        .or_else(|| row.checked_sub(1).and_then(|row| walk(input, row, col, Heading::North)))
        .or_else(|| col.checked_sub(1).and_then(|col| walk(input, row, col, Heading::West)))
        .unwrap()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut start = (0, 0);
    // let mut visited = HashSet::new();
    let input = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if let Some(col) = line.find('S') {
                start = (row, col);
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let loop_dist = begin(&input, start.0, start.1);

    (loop_dist / 2, 0)
}

#[test]
fn test1() {
    let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
    assert_eq!(run(input).0, 4);
}

#[test]
fn test2() {
    let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    assert_eq!(run(input).0, 8);
}
