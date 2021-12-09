trait IsValley {
    fn is_valley(&self, x: usize, y: usize) -> Option<u32>;
}

impl IsValley for Vec<Vec<u32>> {
    fn is_valley(&self, x: usize, y: usize) -> Option<u32> {
        let (x1, x2) = (
            x.checked_sub(1).unwrap_or_default(),
            (x + 1).min(self.len() - 1),
        );
        let (y1, y2) = (
            y.checked_sub(1).unwrap_or_default(),
            (y + 1).min(self[x].len() - 1),
        );

        for xi in x1..=x2 {
            for yi in y1..=y2 {
                if self[xi][yi] < self[x][y] {
                    return None;
                }
            }
        }

        Some(self[x][y])
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let map: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let d9p1: u32 = (0..map.len())
        .map(|x| (0..map[x].len()).map(move |y| (x, y)))
        .flatten()
        .filter_map(|(x, y)| map.is_valley(x, y).map(|h| h + 1))
        .sum();

    (d9p1 as usize, 0)
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
    assert_eq!(run(input), (15, 0));
}
