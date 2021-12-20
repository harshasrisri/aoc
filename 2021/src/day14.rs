use std::collections::HashMap;

fn stepping_fn(polymer: String, insertions: &HashMap<&[u8], char>) -> String {
    let last = polymer.chars().last().unwrap();
    let mut polymer = polymer
        .as_bytes()
        .windows(2)
        .map(|w| {
            let ins = insertions.get(w).unwrap();
            let mut ret = String::new();
            ret.push(w[0] as char);
            ret.push(*ins);
            ret
        })
    .collect::<String>();
    polymer.push(last);
    polymer
}

fn counting_fn(polymer: &str) -> (usize, usize) {
    let counts = polymer
        .chars()
        .fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0_usize) += 1;
            map
        })
    .into_iter()
        .map(|(_,v)| v)
        .collect::<Vec<_>>();

    counts.into_iter().fold((usize::MAX, 0), |(min, max), c| (min.min(c), max.max(c)))
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut input = input.trim().lines();
    let template = input.next().unwrap().to_string();
    let insertions = input
        .skip(1)
        .filter_map(|line| line.split_once(" -> "))
        .map(|(k, v)| (k.as_bytes(), v.chars().next().unwrap()))
        .collect::<HashMap<_, _>>();

    let polymer = (0..10).fold(template, |polymer, _| stepping_fn(polymer, &insertions));
    let (min, max) = counting_fn(&polymer);
    let d14p1 = max - min;

    (d14p1, 0)
}

#[test]
fn test() {
    let input = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
    assert_eq!(run(input), (1588, 2188189693529));
}
