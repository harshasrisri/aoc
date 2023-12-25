use std::{sync::OnceLock, collections::{HashMap, HashSet}};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

type Rules = HashMap<char, HashMap<Heading, Vec<Heading>>>;
static RULES: OnceLock<Rules> = OnceLock::new();

fn rules() -> &'static Rules {
    RULES.get_or_init(|| {
        let rules = [
            ('|', Heading::North, vec![Heading::North]),
            ('|', Heading::South, vec![Heading::South]),
            ('|', Heading::East, vec![Heading::North, Heading::South]),
            ('|', Heading::West, vec![Heading::North, Heading::South]),
            ('-', Heading::North, vec![Heading::East, Heading::West]),
            ('-', Heading::South, vec![Heading::East, Heading::West]),
            ('-', Heading::East, vec![Heading::East]),
            ('-', Heading::West, vec![Heading::West]),
            ('/', Heading::North, vec![Heading::East]),
            ('/', Heading::South, vec![Heading::West]),
            ('/', Heading::East, vec![Heading::North]),
            ('/', Heading::West, vec![Heading::South]),
            ('\\', Heading::North, vec![Heading::West]),
            ('\\', Heading::South, vec![Heading::East]),
            ('\\', Heading::East, vec![Heading::South]),
            ('\\', Heading::West, vec![Heading::North]),
            ('.', Heading::North, vec![Heading::North]),
            ('.', Heading::South, vec![Heading::South]),
            ('.', Heading::East, vec![Heading::East]),
            ('.', Heading::West, vec![Heading::West]),
        ];
        let mut rule_map = HashMap::new();
        for (item, incoming, outgoing) in rules {
            rule_map.entry(item)
                .and_modify(|mp: &mut HashMap<_, _>| { mp.insert(incoming, outgoing.clone()); })
                .or_insert(HashMap::from([(incoming, outgoing)]));
        }
        rule_map
    })
}

fn beam(input: &[Vec<char>], row: usize, col: usize, heading: Heading, visited: &mut HashSet<(usize, usize, Heading)>) -> Option<()> {
    if !visited.insert((row, col, heading)) {
        return Some(());
    }

    for next_heading in rules().get(&input[row][col])?.get(&heading)?.iter() {
        let (next_row, next_col) = match next_heading {
            Heading::South => (Some(row + 1), Some(col)),
            Heading::North => (row.checked_sub(1), Some(col)),
            Heading::East => (Some(row), Some(col + 1)),
            Heading::West => (Some(row), col.checked_sub(1)),
        };
        if let Some((next_row, next_col)) = next_row.zip(next_col) {
            if input.get(next_row).and_then(|r| r.get(next_col)).is_some() {
                beam(input, next_row, next_col, *next_heading, visited)?;
            }
        }
    }
    Some(())
}

fn illumination(input: &[Vec<char>], row: usize, col: usize, heading: Heading) -> usize {
    let mut visited = HashSet::new();
    beam(input, row, col, heading, &mut visited);
    visited.iter().map(|(row, col, _)| (row, col)).collect::<HashSet<_>>().len()
}

pub fn run(input: &'static str) -> (usize, usize) {
    let input = input.trim().lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p2 = (0..input[0].len()).map(|col| (0, col, Heading::South))
        .chain((0..input[0].len()).map(|col| (input.len() - 1, col, Heading::North)))
        .chain((0..input.len()).map(|row| (row, 0, Heading::East)))
        .chain((0..input.len()).map(|row| (row, input[0].len() - 1, Heading::West)))
        .map(|(row, col, heading)| illumination(&input, row, col, heading))
        .max().unwrap();

    (illumination(&input, 0, 0, Heading::East), p2)
}

#[test]
fn test1() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    assert_eq!(run(input), (46, 51));
}
