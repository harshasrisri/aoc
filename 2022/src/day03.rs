use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|items| {
            let len = items.len();
            let h1 = items.chars().take(len / 2).collect::<HashSet<_>>();
            let h2 = items.chars().skip(len / 2).collect::<HashSet<_>>();
            *h1.intersection(&h2).next().unwrap()
        })
        .fold(0, |acc, x| {
            acc + if x.is_lowercase() {
                x as usize - 'a' as usize + 1
            } else {
                x as usize - 'A' as usize + 1 + 26
            }
        });

    let p2 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let c0 = chunk[0].chars().to_owned().collect::<HashSet<_>>();
            let c1 = chunk[1].chars().to_owned().collect::<HashSet<_>>();
            let c01 = c0.intersection(&c1).copied().collect::<HashSet<_>>();
            let c2 = chunk[2].chars().to_owned().collect::<HashSet<_>>();
            *c2.intersection(&c01).next().unwrap()
        })
        .fold(0, |acc, x| {
            acc + if x.is_lowercase() {
                x as usize - 'a' as usize + 1
            } else {
                x as usize - 'A' as usize + 1 + 26
            }
        });

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    assert_eq!(run(input), (157, 70));
}
