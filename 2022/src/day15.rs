use std::collections::HashSet;

use sscanf::sscanf;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_xy(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn man_dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct SBPair {
    sensor: Point,
    beacon: Point,
    man_dist: usize,
}

impl SBPair {
    fn from_ends(sensor: Point, beacon: Point) -> SBPair {
        let man_dist = sensor.man_dist(&beacon);
        SBPair { sensor, beacon, man_dist }
    }

    fn points_on_hline(&self, y: isize) -> impl Iterator<Item = Point> {
        let overshoot = self.man_dist as isize - self.sensor.y.abs_diff(y) as isize;
        (self.sensor.x - overshoot..=self.sensor.x + overshoot).map(move |x| Point::from_xy(x, y))
    }
}


const P1_HLINE: isize = if cfg!(test) { 10 } else { 2000000 };

fn p1(sb_map: &[SBPair]) -> usize {
    let beacons_on_hline = sb_map
        .iter()
        .filter_map(|sb| if sb.beacon.y == P1_HLINE { Some(sb.beacon.clone()) } else { None })
        .collect::<HashSet<_>>()
        .len();

    let points_on_hline = sb_map
        .iter()
        .flat_map(|sb| sb.points_on_hline(P1_HLINE))
        .collect::<HashSet<_>>()
        .len();

    points_on_hline - beacons_on_hline
}

pub fn run(input: &'static str) -> (usize, usize) {
    // let (mut xmn, mut xmx, mut ymn, mut ymx) = (0, 0, 0, 0);
    let sb_map = input
        .lines()
        .map(|line| sscanf!(line, "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}").unwrap())
        .map(|(sx, sy, bx, by)| SBPair::from_ends(Point::from_xy(sx, sy), Point::from_xy(bx, by)))
        // .inspect(|sb| eprintln!("Parsed: {:?}", sb))
        // .inspect(|sb| {
        //     xmn = xmn.min(sb.sensor.x - sb.man_dist as isize);
        //     xmx = xmx.max(sb.sensor.x + sb.man_dist as isize);
        //     ymn = ymn.min(sb.sensor.y - sb.man_dist as isize);
        //     ymx = ymx.max(sb.sensor.y + sb.man_dist as isize);
        // })
        .collect::<Vec<_>>();

    // eprintln!("{xmn}, {xmx}, {ymn}, {ymx}");
    
    (p1(&sb_map), 0)
}

#[test]
fn test() {
    let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
    assert_eq!(run(input), (26, 0));
}
