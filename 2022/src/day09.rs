use sscanf::sscanf;
use std::collections::HashSet;

struct Rope {
    points: Vec<(isize, isize)>,
}

impl Rope {
    fn new(points: usize) -> Rope {
        Rope {
            points: vec![(0, 0); points],
        }
    }

    fn move_head(&mut self, dir: char) -> (isize, isize) {
        let head = &self.points[0];
        self.points[0] = match dir {
            'L' => (head.0 + 1, head.1),
            'R' => (head.0 - 1, head.1),
            'U' => (head.0, head.1 + 1),
            'D' => (head.0, head.1 - 1),
            x => panic!("Invalid dir {x}"),
        };

        let mut head = self.points[0];
        self.points.iter_mut().skip(1).for_each(|tail| {
            let reduce = |d: isize| d.cmp(&0) as isize;
            let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);
            (tail.0, tail.1) = match (dx.abs(), dy.abs()) {
                (adx, ady) if adx < 2 && ady < 2 => (tail.0, tail.1),
                (adx, ady) if adx <= 2 || ady <= 2 => (tail.0 + reduce(dx), tail.1 + reduce(dy)),
                (x, y) => panic!("Invalid distance {x},{y}"),
            };
            head = *tail;
        });
        self.points.last().cloned().unwrap()
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

    let mut rope = Rope::new(2);
    let p1 = head_moves
        .chars()
        .map(|dir| rope.move_head(dir))
        .collect::<HashSet<_>>()
        .len();

    let mut long_rope = Rope::new(10);
    let p2 = head_moves
        .chars()
        .map(|dir| long_rope.move_head(dir))
        .collect::<HashSet<_>>()
        .len();

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    assert_eq!(run(input), (88, 36));
}
