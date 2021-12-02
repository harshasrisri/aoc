fn main() {
    let mut polymer = String::new();
    std::io::stdin().read_line(&mut polymer).unwrap();
    let polymer = polymer.into_bytes();

    let reacted = react(&polymer, 0);
    println!("Part1 : {}", reacted.len());

    let mut min_skip = 0 as u8;
    let mut min_poly = reacted;
    for skip in 65..91 as u8 {
        let improved = react(&polymer, skip);
        if improved.len() <= min_poly.len() {
            min_poly = improved;
            min_skip = skip;
        }
    }
    println!("Part2 : {} - {}", min_skip as char, min_poly.len());
}

fn react(polymer: &[u8], skip: u8) -> String {
    let mut reacted = Vec::new();

    for &unit in polymer {
        if unit == skip || unit == skip + 32 {
            continue;
        }

        if reacted.is_empty() {
            reacted.push(unit);
            continue;
        };

        let r = i32::from(*reacted.last().unwrap());
        let p = i32::from(unit);

        if (r - p).abs() == 32 {
            reacted.pop();
        } else {
            reacted.push(unit);
        }
    }
    String::from_utf8(reacted).unwrap()
}
