use nom::{IResult, multi::{separated_list0, many0}, combinator::{map_res, eof}, character::complete::{digit0, not_line_ending, space0, line_ending}, bytes::complete::tag, sequence::{terminated, delimited, tuple}, branch::alt};

#[derive(Debug)]
struct MapEntry {
    dst: usize,
    src: usize,
    rng: usize,
}

impl MapEntry {
    fn map(&self, num: usize) -> Option<usize> {
        if num >= self.src && num < self.src + self.rng {
            Some(self.dst + num - self.src)
        } else {
            None
        }
    }

    fn parse(input: &str) -> IResult<&str, MapEntry> {
        let parse_number = |input| -> IResult<&str, usize> { delimited(space0, map_res(digit0, str::parse::<usize>), space0)(input) };
        tuple((parse_number, parse_number, parse_number))(input).map(|(input, (dst, src, rng))| (input, MapEntry { dst, src, rng }))
    }
}

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn map(&self, num: usize) -> usize {
        self.entries.iter().find_map(|entry| entry.map(num)).unwrap_or(num)
    }

    fn parse(input: &str) -> IResult<&str, Map> {
        let (input, _title) = terminated(not_line_ending, tag("\n"))(input)?;
        let (remaining, entries) = terminated(
            many0(terminated(MapEntry::parse, alt((line_ending, eof)))),
            alt((line_ending, eof))
        )(input)?;
        Ok((remaining, Map { entries }))
    }
}

struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Almanac> {
        many0(Map::parse)(input).map(|(input, maps)| (input, Almanac { maps }))
    }

    fn seed_to_locn(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |map_val, map_fn| { map_fn.map(map_val) })
    }
}

fn parse_almanac(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;
    terminated(
        separated_list0( tag(" "), map_res(digit0, str::parse::<usize>)),
        tag("\n\n")
    )(input)
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (input, seeds) = parse_almanac(input).unwrap();
    let (_, almanac) = Almanac::parse(input).unwrap();
    let p1 = seeds.iter().map(|seed| almanac.seed_to_locn(*seed)).min().unwrap();
    let p2 = seeds.chunks(2).flat_map(|c| (c[0]..c[0]+c[1]).map(|seed| almanac.seed_to_locn(seed))).min().unwrap();
    
    (p1, p2)
}

#[test]
fn test() {
    let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    assert_eq!(run(input), (35, 46));
}
