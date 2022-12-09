use std::collections::HashSet;
use sscanf::sscanf;

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Default, Clone, Debug)]
struct Rope {
    points: Vec<Point>,
}

impl Rope {
    fn new(points: usize) -> Rope {
        Rope { points: vec![ Point::default(); points ] }
    }

    fn move_head(&mut self, dir: char) -> Point {
        let head = &self.points[0];
        self.points[0] = match dir {
            'L' => Point { x: head.x + 1, y: head.y },
            'R' => Point { x: head.x - 1, y: head.y },
            'U' => Point { x: head.x    , y: head.y + 1 },
            'D' => Point { x: head.x    , y: head.y - 1 },
            x => panic!("Invalid dir {x}") ,
        };

        let mut head = &self.points[0];
        for i in 1..self.points.len() {
            let tail = &self.points[i];
            let reduce = |d: isize| { d.cmp(&0) as isize };
            let (dx, dy) = (head.x - tail.x, head.y - tail.y);
            self.points[i] = match (dx.abs(), dy.abs()) {
                (adx, ady) if adx < 2 && ady < 2 => Point { x: tail.x, y: tail.y},
                (adx, ady) if adx <= 2 || ady <= 2 => Point { x: tail.x + reduce(dx), y: tail.y + reduce(dy)},
                (x, y) => panic!("Invalid distance {x},{y}"),
            };
            head = &self.points[i];
        }
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
        .as_str()
        .chars()
        .map(|dir| rope.move_head(dir))
        .collect::<HashSet<_>>()
        .len();

    let mut long_rope = Rope::new(10);
    let p2 = head_moves
        .as_str()
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
