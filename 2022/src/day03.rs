use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|items| {
            let len = items.len();
            let h1 = items.chars().take(len/2).collect::<HashSet<_>>();
            let h2 = items.chars().skip(len/2).collect::<HashSet<_>>();
            *h1.intersection(&h2).next().unwrap() as usize
        })
        .fold(0, |acc, x| {
            acc +
                if x >= 'a' as usize && x <= 'z' as usize {
                    x - 'a' as usize + 1
                } else if x >= 'A' as usize && x <= 'Z' as usize {
                    x - 'A' as usize + 1 + 26
                } else {
                    0
                }
        });

    let p2 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let c1 = chunk[0].chars().to_owned().collect::<HashSet<_>>();
            let c2 = chunk[1].chars().to_owned().collect::<HashSet<_>>();

            let c12 = c1.intersection(&c2).copied().collect::<HashSet<_>>();
            let c3 = chunk[2].chars().to_owned().collect::<HashSet<_>>();
            *c3.intersection(&c12).next().unwrap() as usize
        })
    .fold(0, |acc, x| {
        acc +
            if x >= 'a' as usize && x <= 'z' as usize {
                x - 'a' as usize + 1
            } else if x >= 'A' as usize && x <= 'Z' as usize {
                x - 'A' as usize + 1 + 26
            } else {
                0
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
    assert_eq!(run(input), (157,70));
}
