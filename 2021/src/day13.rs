pub fn run(input: &'static str) -> (usize, usize) {
    (0,0)
}

#[test]
fn test() {
    let input = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
    assert_eq!(run(input), (17, 0));
}
