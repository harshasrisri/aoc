trait ValleyProps {
    fn is_valley(&self, x: usize, y: usize) -> Option<u32>;
    fn basin_size(&mut self, x: usize, y: usize) -> Option<usize>;
}

impl ValleyProps for Vec<Vec<u32>> {
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

    fn basin_size(&mut self, x: usize, y: usize) -> Option<usize> {
        if self[x][y] == 9 {
            return None;
        }

        let (x1, x2) = (
            x.checked_sub(1).unwrap_or_default(),
            (x + 1).min(self.len() - 1),
        );
        let (y1, y2) = (
            y.checked_sub(1).unwrap_or_default(),
            (y + 1).min(self[x].len() - 1),
        );

        let mut basin = 1;
        self[x][y] = 9;
        for xi in x1..=x2 {
            basin += self.basin_size(xi, y).unwrap_or_default();
        }

        for yi in y1..=y2 {
            basin += self.basin_size(x, yi).unwrap_or_default();
        }

        Some(basin)
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let mut map: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (height, width) = (map.len(), map[0].len());

    let coords = (0..height)
        .map(|x| (0..width).map(move |y| (x, y)))
        .flatten();

    let d9p1: u32 = coords
        .clone()
        .filter_map(|(x, y)| map.is_valley(x, y).map(|h| h + 1))
        .sum();

    let mut basins = coords
        .filter_map(|(x, y)| map.basin_size(x, y))
        .collect::<Vec<_>>();

    basins.sort_by(|a, b| b.cmp(a));

    let d9p2 = basins.iter().take(3).product();

    (d9p1 as usize, d9p2)
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
    assert_eq!(run(input), (15, 1134));
}
