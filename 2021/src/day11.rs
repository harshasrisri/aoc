trait FlashingOctopii {
    fn elapse_day(&mut self);
    fn flash_octopus(&mut self, x: usize, y: usize);
    fn num_flashes(&self) -> usize;
    fn print(&self);
}

impl FlashingOctopii for Vec<Vec<u32>> {
    fn elapse_day(&mut self) {
        for row in self.iter_mut() {
            for octopus in row.iter_mut() {
                *octopus += 1;
            }
        }
    }

    fn flash_octopus(&mut self, x: usize, y: usize) {
        if self[x][y] <= 9 {
            return;
        }

        self[x][y] = 0;

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
                if (xi, yi) == (x, y) || self[xi][yi] == 0 {
                    continue;
                } else {
                    self[xi][yi] += 1;
                    self.flash_octopus(xi, yi);
                }
            }
        }
    }

    fn num_flashes(&self) -> usize {
        self.iter()
            .map(|row| row.iter())
            .flatten()
            .filter(|energy| **energy == 0)
            .count()
    }

    fn print(&self) {
        for row in self.iter() {
            for octopus in row.iter() {
                eprint!("{:2} ", octopus);
            }
            eprintln!();
        }
        eprintln!();
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

    let d11p1 = (0..100)
        .map(|_| {
            map.elapse_day();
            coords.clone().for_each(|(x, y)| map.flash_octopus(x, y));
            map.num_flashes()
        })
        .sum::<usize>();

    let d11p2 = 100 + // 1st 100 for part1
        (100..)
        .map(|_| {
            map.elapse_day();
            coords.clone().for_each(|(x, y)| map.flash_octopus(x, y));
            map.num_flashes()
        })
        .take_while(|flashes| *flashes < height * width)
        .count()
        + 1; // take_while skips the last condition

    (d11p1, d11p2)
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
    assert_eq!(run(input), (1656, 195));
}
