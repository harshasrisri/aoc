use std::collections::HashMap;

pub fn run(input: &'static str) -> (usize, usize) {
    let directions = input.lines().next().unwrap().chars().map(|d| if d == 'L' { 0 } else { 1 }).collect::<Vec<usize>>();
    let dir_map = input
        .lines()
        .skip(2)
        .filter_map(|line| sscanf::sscanf!(line, "{str} = ({str}, {str})").ok())
        .map(|(src, lft, rgt)| (src, vec![lft, rgt]))
        .collect::<HashMap<_, _>>();

    let get_count = |start, end: &str| -> usize {
        let mut cur = start;
        directions.iter().cycle().take_while(|d| { cur = dir_map[cur][**d]; !cur.ends_with(end) }).count() + 1
    };

    let p1 = dir_map.get("AAA").map(|_| get_count("AAA", "ZZZ")).unwrap_or_default();
    let p2 = dir_map.keys().filter(|key| key.ends_with('A'))
        .map(|&start| get_count(start, "Z"))
        .fold(1, num::integer::lcm);

    (p1, p2)
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
