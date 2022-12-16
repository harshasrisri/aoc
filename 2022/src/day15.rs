use ranges::{GenericRange, OperationResult};
use sscanf::sscanf;
use std::{collections::HashSet, ops::RangeInclusive};

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

    fn points_on_hline(&self, y: isize) -> RangeInclusive<isize> {
        let overshoot = self.man_dist as isize - self.sensor.y.abs_diff(y) as isize;
        self.sensor.x - overshoot..=self.sensor.x + overshoot
    }

    fn points_on_hline_right_of(&self, line: isize, point: isize) -> Option<RangeInclusive<isize>> {
        let overshoot = self.man_dist as isize - self.sensor.y.abs_diff(line) as isize;
        let start = point.max(self.sensor.x - overshoot);
        let end = self.sensor.x + overshoot;
        if start <= end { Some(start..=end) } else { None }
    }

    fn points_on_vline_below(&self, line: isize, point: isize) -> Option<RangeInclusive<isize>> {
        let overshoot = self.man_dist as isize - self.sensor.x.abs_diff(line) as isize;
        let start = point.max(self.sensor.y - overshoot);
        let end = self.sensor.y + overshoot;
        if start <= end { Some(start..=end) } else { None }
    }
}

const P1_HLINE: isize = if cfg!(test) { 10 } else { 2000000 };

fn p1(sb_map: &[SBPair]) -> usize {
    let beacons_on_hline = sb_map
        .iter()
        .filter_map(|sb| if sb.beacon.y == P1_HLINE { Some(sb.beacon.clone()) } else { None } )
        .collect::<HashSet<_>>()
        .len();

    let points_on_hline = sb_map
        .iter()
        .flat_map(|sb| sb.points_on_hline(P1_HLINE))
        .collect::<HashSet<_>>()
        .len();

    points_on_hline - beacons_on_hline
}

const P2_RANGE: isize = if cfg!(test) { 20 } else { 4000000 };

struct Ranges {
    ranges: Vec<GenericRange<isize>>,
}

impl Ranges {
    fn from_range(range: RangeInclusive<isize>) -> Ranges {
        Ranges { ranges: vec![GenericRange::from(range)] }
    }

    fn remove(&mut self, hole: RangeInclusive<isize>) {
        let hole = GenericRange::from(hole);
        self.ranges = self
            .ranges
            .iter()
            .flat_map(|range| match range.difference(hole) {
                OperationResult::Empty => vec![].into_iter(),
                OperationResult::Single(r) => vec![r].into_iter(),
                OperationResult::Double(l, r) => vec![l, r].into_iter(),
            })
            .collect::<Vec<_>>();
    }
}

fn p2(sb_map: &[SBPair]) -> usize {
    let (mut b_row, mut b_col): (Option<isize>, Option<isize>) = (None, None);

    // Move along diagonal until...
    for d in 0..=P2_RANGE {

        // ... we find a row/col which contains the beacon. But how?
        if b_row.is_some() && b_col.is_some() {
            break;
        }

        // Start with a full range from diagonal till edge along row/col
        let mut row_occ = Ranges::from_range(d..=P2_RANGE);
        let mut col_occ = Ranges::from_range(d..=P2_RANGE);

        // Then remove the points occupied by each sensor-beacon pair along given row/col
        for sb in sb_map {
            sb.points_on_hline_right_of(d, d).map(|r| row_occ.remove(r)).unwrap_or_default();
            sb.points_on_vline_below(d, d).map(|r| col_occ.remove(r)).unwrap_or_default();
        }

        // a row/col should be full consumed, or have no more than a single hole
        match (row_occ.ranges.len(), col_occ.ranges.len()) {
            // no holes, next diagonal
            (0, 0) => continue,

            // 1 hole on our row
            (1, 0) => {
                let hole = row_occ.ranges.pop().unwrap();
                assert!(hole.is_singleton(), "remaining region not a singleton: {:?}", hole);
                (b_row, b_col) = (Some(hole.into_iter().next().unwrap()), Some(d));
            }

            // 1 hole on our col
            (0, 1) => {
                let hole = col_occ.ranges.pop().unwrap();
                assert!(hole.is_singleton(), "remaining region not a singleton: {:?}", hole);
                (b_row, b_col) = (Some(d), Some(hole.into_iter().next().unwrap()));
            }

            // 1 hole on the diagonal
            (1, 1) => {
                let row_hole = row_occ.ranges.pop().unwrap();
                let col_hole = col_occ.ranges.pop().unwrap();
                assert!(row_hole.is_singleton() && col_hole.is_singleton(), "remaining regions not singletons: {:?}, {:?}", row_hole, col_hole);
                assert_eq!(row_hole.into_iter().next().unwrap(), d, "row intersection is not along diagonal");
                assert_eq!(col_hole.into_iter().next().unwrap(), d, "col intersection is not along diagonal");
                (b_row, b_col) = (Some(d), Some(d));
            }

            // more than 1 hole, invalid scenario
            (_, _) => panic!("Invalid number of ranges remaining"),
        }
    }

    let (row, col) = (b_row.unwrap() as usize, b_col.unwrap() as usize);

    row * 4000000 + col
}

pub fn run(input: &'static str) -> (usize, usize) {
    let sb_map = input
        .lines()
        .map(|line| sscanf!(line, "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}").unwrap())
        .map(|(sx, sy, bx, by)| SBPair::from_ends(Point::from_xy(sx, sy), Point::from_xy(bx, by)))
        .collect::<Vec<_>>();

    (p1(&sb_map), p2(&sb_map))
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
    assert_eq!(run(input), (26, 56000011));
}
