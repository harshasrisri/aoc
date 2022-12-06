use std::collections::HashSet;

pub fn run(input: &'static str) -> (usize, usize) {
    let p1 = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find_map(|(pos, bytes)| {
            eprintln!("{}, {:?}", pos, bytes);
            let bytes = bytes.iter().collect::<HashSet<_>>();
            if bytes.len() == 4 {
                Some(pos + 4)
            } else {
                None
            }
        })
        .unwrap();

    (p1, 0)
}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(run(input), (7,0));
}
