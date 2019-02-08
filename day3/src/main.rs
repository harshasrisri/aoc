use std::io::{BufRead, BufReader};
#[macro_use]
extern crate text_io;

fn main() {
    let strings: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|r| r.ok())
        .collect();
    let mut fabric: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut claim_overlaps = vec![false; strings.len()];
    let mut overlap = 0u32;
    for s in &strings {
        // #1 @ 604,670: 22x16
        let (id, x, y, w, h): (i32, usize, usize, usize, usize);
        scan!(s.bytes() => "#{} @ {},{}: {}x{}", id, x, y, w, h);
        for i in x..x + w {
            for j in y..y + h {
                fabric[i][j] = match fabric[i][j] {
                    0 => id,
                    _ => {
                        if fabric[i][j] != -1 {
                            claim_overlaps[(fabric[i][j] - 1) as usize] = true;
                            overlap += 1;
                        }
                        claim_overlaps[(id - 1) as usize] = true;
                        -1
                    }
                }
            }
        }
    }
    println!("overlap = {} sq. in.", overlap);
    println!(
        "ID {} Doesn't overlap",
        1 + &claim_overlaps.iter().position(|&i| !i).unwrap()
    );
}
