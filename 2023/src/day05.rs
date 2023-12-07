use std::marker::PhantomData;

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
        fn parse_number(input: &str) -> IResult<&str, usize> {
            delimited(space0, map_res(digit0, str::parse::<usize>), space0)(input)
        }
        let (remaining, (dst, src, rng)) = tuple((parse_number, parse_number, parse_number))(input)?;
        Ok((remaining, MapEntry { dst, src, rng }))
    }
}

struct Map<S, D> {
    entries: Vec<MapEntry>,
    src_type: PhantomData<S>,
    dst_type: PhantomData<D>,
}

impl<S,D> Map<S,D> {
    fn map(&self, num: usize) -> usize {
        if let Some(mapped) = self.entries.iter().find_map(|entry| entry.map(num)) {
            mapped
        } else {
            num
        }
    }

    fn parse(input: &str) -> IResult<&str, Map<S,D>> {
        let (input, _title) = terminated(not_line_ending, tag("\n"))(input)?;
        let (remaining, entries) = terminated(
            many0(terminated(MapEntry::parse, alt((line_ending, eof)))),
            alt((line_ending, eof))
        )(input)?;
        Ok((remaining, Map { entries, src_type: PhantomData, dst_type: PhantomData }))
    }
}

type Seed = usize;
type Soil = usize;
type Fert = usize;
type Watr = usize;
type Lght = usize;
type Temp = usize;
type Hmdt = usize;
type Locn = usize;

struct Almanac {
    seed_to_soil: Map<Seed, Soil>,
    soil_to_fert: Map<Soil, Fert>,
    fert_to_watr: Map<Fert, Watr>,
    watr_to_lght: Map<Watr, Lght>,
    lght_to_temp: Map<Lght, Temp>,
    temp_to_hmdt: Map<Temp, Hmdt>,
    hmdt_to_locn: Map<Hmdt, Locn>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Almanac> {
        let (input, seed_to_soil) = Map::parse(input)?;
        let (input, soil_to_fert) = Map::parse(input)?;
        let (input, fert_to_watr) = Map::parse(input)?;
        let (input, watr_to_lght) = Map::parse(input)?;
        let (input, lght_to_temp) = Map::parse(input)?;
        let (input, temp_to_hmdt) = Map::parse(input)?;
        let (input, hmdt_to_locn) = Map::parse(input)?;

        Ok((input, Almanac { seed_to_soil, soil_to_fert, fert_to_watr, watr_to_lght, lght_to_temp, temp_to_hmdt, hmdt_to_locn }))
    }

    fn to_soil(&self, seed: Seed) -> Soil { self.seed_to_soil.map(seed) }
    fn to_fert(&self, soil: Soil) -> Fert { self.soil_to_fert.map(soil) }
    fn to_watr(&self, fert: Fert) -> Watr { self.fert_to_watr.map(fert) }
    fn to_lght(&self, watr: Watr) -> Lght { self.watr_to_lght.map(watr) }
    fn to_temp(&self, lght: Lght) -> Temp { self.lght_to_temp.map(lght) }
    fn to_hmdt(&self, temp: Temp) -> Hmdt { self.temp_to_hmdt.map(temp) }
    fn to_locn(&self, hmdt: Hmdt) -> Locn { self.hmdt_to_locn.map(hmdt) }

}

fn parse_almanac(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;
    terminated(
        separated_list0(
            tag(" "), 
            map_res(digit0, str::parse::<usize>)
        ),
        tag("\n\n")
    )(input)
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (input, seeds) = parse_almanac(input).unwrap();
    let (_, almanac) = Almanac::parse(input).unwrap();

    let p1 = seeds
        .iter()
        .map(|seed| almanac.to_soil(*seed))
        .map(|soil| almanac.to_fert(soil))
        .map(|fert| almanac.to_watr(fert))
        .map(|watr| almanac.to_lght(watr))
        .map(|lght| almanac.to_temp(lght))
        .map(|temp| almanac.to_hmdt(temp))
        .map(|hmdt| almanac.to_locn(hmdt))
        .min().unwrap();

    let p2 = seeds
        .chunks(2)
        .flat_map(|c| (c[0]..c[0]+c[1])
            .into_iter()
            .map(|seed| almanac.to_soil(seed))
            .map(|soil| almanac.to_fert(soil))
            .map(|fert| almanac.to_watr(fert))
            .map(|watr| almanac.to_lght(watr))
            .map(|lght| almanac.to_temp(lght))
            .map(|temp| almanac.to_hmdt(temp))
            .map(|hmdt| almanac.to_locn(hmdt))
        )
        .min().unwrap();

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
