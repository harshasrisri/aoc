use sscanf::sscanf;
use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let (s1, e1, s2, e2) = sscanf!(line, "{usize}-{usize},{usize}-{usize}").unwrap();
            let assn1 = (s1..=e1).collect::<HashSet<_>>();
            let assn2 = (s2..=e2).collect::<HashSet<_>>();
            let p1 = assn1.is_subset(&assn2) || assn2.is_subset(&assn1);
            let p2 = assn1.intersection(&assn2).next().is_some();
            (p1, p2)
        })
        .fold((0, 0), |(c1, c2), (p1, p2)| {
            (c1 + p1 as usize, c2 + p2 as usize)
        })
}

#[test]
fn test() {
    let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    assert_eq!(run(input), (2, 4));
}
