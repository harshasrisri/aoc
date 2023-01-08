use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone)]
enum Directions {
    Left,
    Right,
    Down,
}

impl Directions {
    fn from(input: &str) -> impl Iterator<Item = Directions> + '_ {
        input
            .chars()
            .cycle()
            .map(|c| match c {
                '<' => Directions::Left,
                '>' => Directions::Right,
                c => panic!("invalid input - {c}"),
            })
            .intersperse(Directions::Down)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let dirs = Directions::from(input);
    (0,0)
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(run(input), (7, 19));
}
