pub fn run(input: &'static str) -> (usize, usize) {
    let scores = [
        [3, 0, 6],
        [6, 3, 0],
        [0, 6, 3],
    ];
    let p1 = input
        .lines()
        .map(|play| {
            let mut play = play.trim().split(' ');
            let elf = play.next().unwrap().chars().next().unwrap() as usize - 'A' as usize;
            let you = play.next().unwrap().chars().next().unwrap() as usize - 'X' as usize;
            scores[you][elf] + you + 1
        })
        .sum();
    (p1, 0)
}

#[test]
fn test() {
    let input = "A Y
B X
C Z";
    assert_eq!(run(input), (15,0));
}
