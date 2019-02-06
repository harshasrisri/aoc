use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() {
    let mut set = HashSet::new();
    let mut sum: i32 = 0;
    let nums: Vec<i32> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| i32::from_str_radix(&line, 10))
        .filter_map(|res| res.ok())
        .collect();
    println!("{}", nums.iter().sum::<i32>());
    loop {
        for num in &nums {
            sum += num;
            match set.contains(&sum) {
                true => {
                    println!("{}", &sum);
                    return;
                }
                false => set.insert(sum),
            };
        }
    }
}
