use std::collections::HashMap;
use std::io::{BufRead, BufReader};
#[macro_use]
extern crate text_io;

fn main() {
    let mut strings: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|r| r.ok())
        .collect();
    strings.sort();

    let (mut _yr, mut _mo, mut _dd, mut _hr, mut mn, mut id) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut falls, mut _wakes) = (0u32, 0u32);
    let mut sleep_table = HashMap::new();

    for s in &mut strings {
        scan! (s.bytes() => "[{}-{}-{} {}:{}]", _yr, _mo, _dd, _hr, mn);
        match &s[19..24] {
            "Guard" => {
                scan! (s[19..].bytes() => "Guard #{} begins shift", id);
                sleep_table.entry(id).or_insert_with(|| vec![0u32; 60]);
            }
            "falls" => {
                if id == 0 || !sleep_table.contains_key(&id) {
                    panic!("Didn't encounter id before falling asleep");
                }
                falls = mn;
            }
            "wakes" => {
                let mut table = sleep_table.get_mut(&id).unwrap().to_vec();
                for i in falls..mn {
                    table[i as usize] += 1;
                }
                sleep_table.insert(id, table);
            }
            _ => panic!("Marker not Guard, falls or wakes"),
        }
    }
    println!("Strategy 1 - {}", strategy_1(&sleep_table));
    println!("Strategy 2 - {}", strategy_2(&sleep_table));
}

fn strategy_1(sleep_table: &HashMap<u32, Vec<u32>>) -> u32 {
    let sleepiest_guard = {
        let (mut max_key, mut max_sleep) = (0u32, 0u32);
        for (&key, table) in sleep_table {
            let sleep = table.iter().sum::<u32>();
            if max_sleep < sleep {
                max_sleep = sleep;
                max_key = key;
            }
        }
        max_key
    };
    let sleepiest_minute = {
        let (mut max_pos, mut max_count) = (0u32, 0u32);
        for (p, c) in sleep_table
            .get(&sleepiest_guard)
            .unwrap()
            .iter()
            .enumerate()
        {
            if *c > max_count {
                max_pos = p as u32;
                max_count = *c;
            }
        }
        max_pos
    };
    sleepiest_guard * sleepiest_minute
}

fn strategy_2(sleep_table: &HashMap<u32, Vec<u32>>) -> u32 {
    let (mut sleepiest_guard, mut sleepiest_minute, mut max_count) = (0u32, 0u32, 0u32);
    for (&guard, table) in sleep_table {
        for (p, c) in table.iter().enumerate() {
            if *c > max_count {
                sleepiest_minute = p as u32;
                sleepiest_guard = guard;
                max_count = *c;
            }
        }
    }
    sleepiest_guard * sleepiest_minute
}
