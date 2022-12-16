use std::{collections::HashSet, ops::RangeInclusive};
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

    fn contains(&self, point: &Point) -> bool {
        self.sensor.man_dist(point) <= self.man_dist
    }

    fn points_on_hline(&self, y: isize) -> RangeInclusive<isize> {
        let overshoot = self.man_dist as isize - self.sensor.y.abs_diff(y) as isize;
        self.sensor.x - overshoot..=self.sensor.x + overshoot
    }

    fn points_on_hline_right_of(&self, line: isize, point: isize) -> Option<RangeInclusive<isize>> {
        let overshoot = self.man_dist as isize - self.sensor.y.abs_diff(line) as isize;
        let start = point.max(self.sensor.x - overshoot);
        let end = self.sensor.x + overshoot;
        // eprint!("points on hline {line} right of {point} for {:?} -> {} - ", self.sensor, self.man_dist);
        if start <= end {
            // eprintln!("{start}..{end}");
            Some(start..=end)
        } else {
            // eprintln!("None");
            None
        }
    }

    fn points_on_vline_below(&self, line: isize, point: isize) -> Option<RangeInclusive<isize>> {
        let overshoot = self.man_dist as isize - self.sensor.x.abs_diff(line) as isize;
        let start = point.max(self.sensor.y - overshoot);
        let end = self.sensor.y + overshoot;
        // eprint!("points on vline {line} below    {point} for {:?} -> {} - ", self.sensor, self.man_dist);
        if start <= end {
            // eprintln!("{start}..{end}");
            Some(start..=end)
        } else {
            // eprintln!("None");
            None
        }
    }
}

trait RangeOps {
    fn coalesces_with(&self, other: &Self) -> bool;
    fn coalesce(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn dmz(&self, other: &Self) -> Option<Self> where Self: Sized;
    fn truncate(&self, start: isize, end: isize) -> Self;
}

impl RangeOps for RangeInclusive<isize> {
    fn coalesces_with(&self, other: &Self) -> bool {
        other.start() - self.end() <= 1 || self.start() - other.end() <= 1
    }

    fn coalesce(&self, other: &Self) -> Option<Self> where Self: Sized {
        if !self.coalesces_with(&other) {
            None
        } else {
            let start = self.start().min(other.start());
            let end = self.end().max(other.end());
            Some(*start..=*end)
        }
    }

    fn dmz(&self, other: &Self) -> Option<Self> where Self: Sized {
        if self.coalesces_with(&other) {
            None
        } else {
            if self.end() < other.start() {
                Some(self.end()+1..=other.start()-1)
            } else if other.end() < self.start() {
                Some(other.end()+1..=self.start()-1)
            } else {
                panic!("Invalid ranges for dmz: {:?} <=> {:?}", self, other)
            }
        }
    }

    fn truncate(&self, start: isize, end: isize) -> Self {
        *self.start().max(&start)..=*self.end().min(&end)
    }
}

#[derive(Debug)]
enum RangeType {
    Occ(RangeInclusive<isize>),
    Dmz(RangeInclusive<isize>),
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

    if cfg!(test) {
        for y in 0..=20 {
            for x in 0..=20 {
                let point = Point {x, y};
                eprint!("{}", 
                        if sb_map.iter().any(|sb| sb.contains(&point)) {
                            '.'
                        } else {
                            '#'
                        })
            }
            eprintln!();
        }
    }

    points_on_hline - beacons_on_hline
}

const P2_RANGE: isize = if cfg!(test) { 20 } else { 4000000 };

fn condense(ranges: Vec<RangeInclusive<isize>>) -> RangeType {
    let mut first: Option<RangeInclusive<isize>> = None;
    let mut second: Option<RangeInclusive<isize>> = None;
    for range in ranges.into_iter() {
        (first, second) = match (first, second) {
            (None, None) => (Some(range), None),
            (Some(f), None) if f.coalesces_with(&range) => (f.coalesce(&range), None),
            (Some(f), None) if !f.coalesces_with(&range) => (Some(f), Some(range)),
            (Some(f), Some(s)) => (Some(f), s.coalesce(&range)),
            _ => panic!("Invalid range overlapping scenario"),
        };
    }
    match (first, second) {
        (Some(f), None) => RangeType::Occ(f.truncate(0, P2_RANGE)),
        (Some(f), Some(s)) if f.coalesces_with(&s) => RangeType::Occ(f.coalesce(&s).unwrap().truncate(0, P2_RANGE)),
        (Some(f), Some(s)) if !f.coalesces_with(&s) => RangeType::Dmz(f.dmz(&s).unwrap().truncate(0, P2_RANGE)),
        _ => panic!("Invalid range overlapping scenario"),
    }
}

fn p2(sb_map: &[SBPair]) -> usize {
    let (mut b_row, mut b_col): (Option<isize>, Option<isize>) = (None, None);

    for d in 0..=P2_RANGE {
        if b_row.is_some() && b_col.is_some() {
            break;
        }

        let mut row_occ = Vec::new();
        let mut col_occ = Vec::new();

        for sb in sb_map {
            if b_row.is_none() {
                if let Some(r) = sb.points_on_hline_right_of(d, d) {
                    row_occ.push(r);
                }
            }

            if b_col.is_none() {
                if let Some(c) = sb.points_on_vline_below(d, d) {
                    col_occ.push(c);
                }
            }
        }

        if b_row.is_none() {
            row_occ.sort_by(|a, b| a.start().cmp(b.start()));
            let row_occ = condense(row_occ);
            b_row = match row_occ {
                RangeType::Occ(r) if r.end() - r.start() == P2_RANGE - d => None,
                RangeType::Occ(r) if r.start() != &d => Some(d),
                RangeType::Occ(r) if r.end() != &P2_RANGE => Some(P2_RANGE),
                RangeType::Dmz(r) if r.start() == r.end() => Some(*r.start()),
                r => panic!("Invalid range overlapping scenario row: {:?}", r),
            }
        }

        if b_col.is_none() {
            col_occ.sort_by(|a, b| a.start().cmp(b.start()));
            let col_occ = condense(col_occ);
            b_col = match col_occ {
                RangeType::Occ(r) if r.end() - r.start() == P2_RANGE - d => None,
                RangeType::Occ(r) if r.start() != &d => Some(d),
                RangeType::Occ(r) if r.end() != &P2_RANGE => Some(P2_RANGE),
                RangeType::Dmz(r) if r.start() == r.end() => Some(*r.start()),
                r => panic!("Invalid range overlapping scenario col: {:?}", r),
            }
        }
    }

    let (row, col) = (b_row.unwrap(), b_col.unwrap());
    eprintln!("{row}, {col}");

    (row * P2_RANGE + col) as usize
}

pub fn run(input: &'static str) -> (usize, usize) {
    // let (mut xmn, mut xmx, mut ymn, mut ymx) = (0, 0, 0, 0);
    let mut sb_map = input
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

    sb_map.sort_by(|a, b| (a.sensor.x + a.sensor.y).cmp(&(b.sensor.x + b.sensor.y)));

    // eprintln!("{xmn}, {xmx}, {ymn}, {ymx}");
    
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
