pub fn run(input: &'static str) -> (usize, usize) {
    let scores = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

    let results = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

    input
        .lines()
        .map(|play| {
            let mut play = play.chars().filter(|c| !c.is_whitespace());

            let elf = play.next().unwrap() as usize - 'A' as usize;
            let you = play.next().unwrap() as usize - 'X' as usize;
            let p1 = scores[elf][you] + you + 1;

            let result = you;
            let p2 = results[elf][result] + (result * 3);

            (p1, p2)
        })
        .fold((0, 0), |(s1, s2), (p1, p2)| (s1 + p1, s2 + p2))
}

#[test]
fn test() {
    let input = "\
A Y
B X
C Z
";
    assert_eq!(run(input), (15, 12));
}
