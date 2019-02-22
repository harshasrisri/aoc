#[macro_use] extern crate text_io;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap,HashSet};

fn main() {
    let mut deps = HashMap::new();
    for line in BufReader::new(std::io::stdin()).lines().filter_map(|r| r.ok()) {
        let (src, dst) : (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", src, dst);
        deps.entry(dst).or_insert(Vec::new()).push(src);
    }
    println!("{:?}", deps);
}
