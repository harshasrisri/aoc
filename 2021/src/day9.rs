pub fn run(input: &'static str) -> (usize, usize) { 
    let _ = input;
    (0, 0)
}

#[test]
fn test() { 
    let input = "
2199943210
3987894921
9856789892
8767896789
9899965678
";
    assert_eq!(run(input), (0,0));
}
