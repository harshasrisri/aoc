pub fn run(input: &'static str) -> (usize, usize) {
    let data = input
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let d1p1 = data.windows(2).filter(|w| w[0] < w[1]).count();

    let d1p2 = data
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();

    (d1p1, d1p2)
}

#[test]
fn test() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    assert_eq!(run(input), (7, 5));
}
