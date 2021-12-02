#[macro_use]
extern crate text_io;
extern crate petgraph;
use petgraph::{graphmap::DiGraphMap, Direction};
use std::io::{BufRead, BufReader};

// Modified Kahn's algorithm borrowed from Wikipedia's Topological Sort entry
//
// L ← Empty list that will contain the sorted elements
// S ← Set of all nodes with no incoming edge in sorted order
// while S is non-empty do
//     remove a node n from S
//     add n to tail of L
//     for each node m with an edge e from n to m do
//         remove edge e from the graph
//         if m has no other incoming edges then
//             insert m into S
//             sort S
// if graph has edges then
//     return error   (graph has at least one cycle)
// else
//     return L   (a topologically sorted order)

fn main() {
    let mut deps = DiGraphMap::new();
    for line in BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|r| r.ok())
    {
        let (src, dst): (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", src, dst);
        deps.add_edge(src, dst, deps.edge_count());
    }

    let mut no_inc: Vec<char> = deps
        .nodes()
        .filter(|&n| deps.neighbors_directed(n, Direction::Incoming).count() == 0)
        .collect();

    no_inc.sort_by(|a, b| b.partial_cmp(&a).unwrap());

    let mut sorted = Vec::new();
    let mut time_taken = 0;

    while !no_inc.is_empty() {
        let n = no_inc.pop().unwrap();
        time_taken += 60 + n as u32;
        sorted.push(n);

        for m in deps.clone().neighbors_directed(n, Direction::Outgoing) {
            deps.remove_edge(n, m);
            if deps.neighbors_directed(m, Direction::Incoming).count() == 0 {
                no_inc.push(m);
                no_inc.sort_by(|a, b| b.partial_cmp(&a).unwrap());
            }
        }
    }

    assert_eq!(deps.all_edges().count(), 0);
    println!(
        "{} in {} secs",
        sorted.into_iter().collect::<String>(),
        time_taken
    );
}
