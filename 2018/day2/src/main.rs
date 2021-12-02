use std::io::{BufRead, BufReader};
fn main() {
    let strings: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|r| r.ok())
        .collect();
    let (mut twos, mut threes) = (0u32, 0u32);
    for s in &strings {
        let mut count = [0u8; 128];
        for c in s.chars().map(|c| c as usize) {
            count[c] += 1;
        }
        if count.contains(&2) {
            twos += 1;
        }
        if count.contains(&3) {
            threes += 1;
        }
    }
    println!("{} {} {}", twos, threes, twos * threes);

    let mut res = String::new();
    'outer: for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            if strings[i]
                .chars()
                .zip(strings[j].chars())
                .filter(|&(c1, c2)| c1 != c2)
                .count()
                == 1
            {
                res = strings[i]
                    .chars()
                    .zip(strings[j].chars())
                    .filter(|&(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect();
                break 'outer;
            }
        }
    }
    println!("Product ID - {}", res);
}
