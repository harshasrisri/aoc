use std::collections::HashMap;

use nom::{sequence::terminated, character::complete::not_line_ending, bytes::complete::tag, IResult};


fn get_directions(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, tag("\n\n"))(input)
}

pub fn run(input: &'static str) -> (usize, usize) {
    let (remaining, directions) = get_directions(input).unwrap();
    let directions = directions.chars().map(|d| if d == 'L' { 0 } else { 1 }).collect::<Vec<usize>>();
    let dir_map = remaining
        .lines()
        .filter_map(|line| sscanf::sscanf!(line, "{str} = ({str}, {str})").ok())
        .map(|(src, lft, rgt)| (src, vec![lft, rgt]))
        .collect::<HashMap<_, _>>();

    let mut cur = "AAA";
    let p1_count = directions.iter().cycle().take_while(|d| { cur = dir_map[cur][**d]; cur != "ZZZ" }).count() + 1;

    (p1_count, 0)
}

#[test]
fn test1() {
    let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(run(input).0, 2);
}

#[test]
fn test2() {
    let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(run(input).0, 6);
}

#[test]
fn test3() {
    let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    assert_eq!(run(input).1, 6);
}
