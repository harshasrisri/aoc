#[macro_use] extern crate text_io;
extern crate petgraph;
use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};
use std::io::{BufRead, BufReader};

fn main() {
    let mut deps = DiGraphMap::new();
    for line in BufReader::new(std::io::stdin()).lines().filter_map(|r| r.ok()) {
        let (src, dst) : (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", src, dst);
        deps.add_edge (src, dst, deps.edge_count());
    }
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
}
