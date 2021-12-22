fn min_risk(vec: &[Vec<usize>], x: usize, y: usize) -> usize {
    if x == vec.len() || y == vec[0].len() {
        return 0;
    }
    eprintln!("{},{} - {}", x, y, vec[x][y]);
    let down = min_risk(vec, x + 1, y);
    let right = min_risk(vec, x, y + 1);

    vec[x][y] + down.min(right)
}

pub fn run(input: &'static str) -> (usize, usize) {
    let input = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c as usize - 0x30).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let d15p1 = min_risk(&input, 0, 0);

    (d15p1, 0)
}

#[test]
fn test() {
    let input = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
    assert_eq!(run(input), (40, 0));
}
