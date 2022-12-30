use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn offset_origin(&mut self, origin: &Point) {
        self.x += origin.x;
        self.y += origin.y;
        self.z += origin.z;
    }
}

const LBF: Point = Point { x: 0, y: 0, z: 0 };
const RBF: Point = Point { x: 1, y: 0, z: 0 };
const LTF: Point = Point { x: 0, y: 1, z: 0 };
const RTF: Point = Point { x: 1, y: 1, z: 0 };
const LBN: Point = Point { x: 0, y: 0, z: 1 };
const RBN: Point = Point { x: 1, y: 0, z: 1 };
const LTN: Point = Point { x: 0, y: 1, z: 1 };
const RTN: Point = Point { x: 1, y: 1, z: 1 };

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Side {
    points: [Point; 4]
}

impl Side {
    fn offset_origin(mut self, origin: &Point) -> Side {
        self.points
            .iter_mut()
            .for_each(|p| p.offset_origin(origin));
        self
    }
}

const LFT: Side = Side { points: [ LBF, LTF, LBN, LTN ] };
const RGT: Side = Side { points: [ RBF, RTF, RBN, RTN ] };
const BOT: Side = Side { points: [ LBF, RBF, LBN, RBN ] };
const TOP: Side = Side { points: [ LTF, RTF, LTN, RTN ] };
const FAR: Side = Side { points: [ LBF, RBF, LTF, RTF ] };
const NER: Side = Side { points: [ LBN, RBN, LTN, RTN ] };

#[derive(Debug)]
struct Cube {
    origin: Point,
    sides: [Side; 6],
}

impl Cube {
    fn new_at(x: usize, y: usize, z: usize) -> Cube {
        let origin = Point { x, y, z };
        let sides = [
            LFT.offset_origin(&origin),
            RGT.offset_origin(&origin),
            BOT.offset_origin(&origin),
            TOP.offset_origin(&origin),
            FAR.offset_origin(&origin),
            NER.offset_origin(&origin),
        ];
        Cube { origin, sides }
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let cubes: Vec<Cube> = input
        .lines()
        .map(|line| sscanf::sscanf!(line, "{usize},{usize},{usize}").unwrap())
        .map(|(x, y, z)| Cube::new_at(x, y, z))
        .collect();

    let mut exposed = HashSet::new();
    cubes
        .iter()
        .flat_map(|cube| cube.sides.iter().cloned())
        .for_each(|side| {
            if exposed.contains(&side) {
                exposed.remove(&side);
                eprintln!("Hidden  side - {:?}", side);
            } else {
                exposed.insert(side.clone());
                eprintln!("Exposed side - {:?}", side);
            }
        });

    (exposed.len(), 0)
}

#[test]
fn test() {
    let input = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
    assert_eq!(run(input), (64,0));
}
