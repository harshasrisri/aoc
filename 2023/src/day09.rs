fn get_prediction(history: &[i32]) -> Option<i32> {
    let new_history = history.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let last = history.last();
    let ret = if new_history.iter().all(|val| *val == 0) {
        last.copied()
    } else {
        last.zip(get_prediction(&new_history)).map(|(l, p)| *l + p)
    };
    ret
}

pub fn run(input: &'static str) -> (usize, usize) {
    let histories = input
        .lines()
        .map(|line| line.split(' ').filter_map(|val| val.parse().ok()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = histories
        .iter()
        .filter_map(|hist| get_prediction(&hist))
        .sum::<i32>();

    (p1 as usize, 0)
}

#[test]
fn test() {
    let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(run(input), (114, 2));
}
