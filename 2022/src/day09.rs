use std::collections::HashSet;

use sscanf::sscanf;

#[derive(Default)]
struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
}

impl Rope {
    fn move_head(&mut self, dir: char) -> Option<(isize, isize)> {
        self.head = match dir {
            'L' => (self.head.0 + 1, self.head.1),
            'R' => (self.head.0 - 1, self.head.1),
            'U' => (self.head.0, self.head.1 + 1),
            'D' => (self.head.0, self.head.1 - 1),
            x => panic!("Invalid dir {x}"),
        };
        self.tail = match (self.head.0 - self.tail.0, self.head.1 - self.tail.1) {
            (0, 0) => self.tail,
            (1, 0) => self.tail,
            (-1, 0) => self.tail,
            (0, 1) => self.tail,
            (0, -1) => self.tail,
            (1, 1) => self.tail,
            (-1, -1) => self.tail,
            (1, -1) => self.tail,
            (-1, 1) => self.tail,
            (-1, 2) => (self.tail.0 - 1, self.tail.1 + 1),  // 11'o clock
            (0, 2) => (self.tail.0, self.tail.1 + 1),       // 12'o clock
            (1, 2) => (self.tail.0 + 1, self.tail.1 + 1),   // 01'o clock
            (2, 1) => (self.tail.0 + 1, self.tail.1 + 1),   // 02'o clock
            (2, 0) => (self.tail.0 + 1, self.tail.1),       // 03'o clock
            (2, -1) => (self.tail.0 + 1, self.tail.1 - 1),  // 04'o clock
            (1, -2) => (self.tail.0 + 1, self.tail.1 - 1),  // 05'o clock
            (0, -2) => (self.tail.0, self.tail.1 - 1),      // 06'o clock
            (-1, -2) => (self.tail.0 - 1, self.tail.1 - 1), // 07'o clock
            (-2, -1) => (self.tail.0 - 1, self.tail.1 - 1), // 08'o clock
            (-2, 0) => (self.tail.0 - 1, self.tail.1),      // 09'o clock
            (-2, 1) => (self.tail.0 - 1, self.tail.1 + 1),  // 10'o clock
            x => panic!("Invalid distance {},{}", x.0, x.1),
        };
        Some(self.tail)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let head_moves: String = input
        .lines()
        .flat_map(|line| {
            let (dir, steps) = sscanf!(line, "{char} {usize}").unwrap();
            std::iter::repeat(dir).take(steps)
        })
        .collect();

    let mut rope = Rope::default();
    let tail_moves = head_moves
        .as_str()
        .chars()
        .filter_map(|dir| rope.move_head(dir))
        .collect::<HashSet<_>>()
        .len();

    (tail_moves, 0)
}

#[test]
fn test() {
    let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    assert_eq!(run(input), (13, 0));
}
