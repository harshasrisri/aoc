use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .lines()
        .map(|line| {
            let (assn1, assn2) = line.split_once(',').unwrap_or_default();

            let (start, end) = assn1.split_once('-').unwrap_or_default();
            let (start, end) = (start.parse::<usize>().unwrap_or_default(), end.parse::<usize>().unwrap_or_default());
            let assn1 = (start..=end).collect::<HashSet<_>>();

            let (start, end) = assn2.split_once('-').unwrap_or_default();
            let (start, end) = (start.parse::<usize>().unwrap_or_default(), end.parse::<usize>().unwrap_or_default());
            let assn2 = (start..=end).collect::<HashSet<_>>();

            assn1.is_subset(&assn2) || assn2.is_subset(&assn1)
        })
        .filter(|is_subset| *is_subset)
        .count();

    (p1, 0)
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
    assert_eq!(run(input), (2,0));
}
