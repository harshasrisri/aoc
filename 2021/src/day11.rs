pub fn run(input: &'static str) -> (usize, usize) {
    (0, 0)
}

#[test]
fn test() {
    let input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
    assert_eq!(run(input), (1656, 0));
}
