fn get_prediction(history: &[i32]) -> (i32, i32) {
    let new_history = history.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let first = *history.first().unwrap();
    let last = *history.last().unwrap();
    if new_history.iter().all(|val| *val == 0) {
        (first, last)
    } else {
        let (pf, pl) = get_prediction(&new_history);
        (first - pf, last + pl)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let histories = input
        .lines()
        .map(|line| line.split(' ').filter_map(|val| val.parse().ok()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (p2, p1) = histories
        .iter()
        .map(|hist| get_prediction(hist))
        .fold((0, 0), |(f_sum, l_sum), (first, last)| {
            (f_sum + first, l_sum + last)
        });

    (p1 as usize, p2 as usize)
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
