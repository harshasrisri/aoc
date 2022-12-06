use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    let find_start = |win_size: usize| -> usize {
        input
            .as_bytes()
            .windows(win_size)
            .enumerate()
            .find_map(|(pos, bytes)| {
                if win_size == bytes.iter().collect::<HashSet<_>>().len() {
                    Some(pos + win_size)
                } else {
                    None
                }
            })
            .unwrap()
    };

    (find_start(4), find_start(14))
}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(run(input), (7, 19));
}
