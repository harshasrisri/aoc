use serde::Deserialize;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
#[serde(untagged)]
enum List {
    Num(usize),
    List(Vec<List>),
}

impl List {
    fn compare(&self, right: &Self) -> Ordering {
        match (self, right) {
            (List::Num(l), List::Num(r)) => l.cmp(r),
            (List::Num(l), List::List(_r)) => List::List(vec![List::Num(*l)]).compare(right),
            (List::List(_l), List::Num(r)) => self.compare(&List::List(vec![List::Num(*r)])),
            (List::List(l), List::List(r)) => {
                for (left, right) in l.iter().zip(r.iter()) {
                    match left.compare(right) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl FromStr for List {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

pub fn run(input: &'static str) -> (usize, usize) {
    let list: Vec<_> = input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            (
                List::from_str(left).unwrap(),
                List::from_str(right).unwrap(),
            )
        })
        .collect();

    let p1 = list
        .iter()
        .zip(1..)
        .filter_map(|((left, right), index)| {
            if left.compare(right) == Ordering::Less {
                Some(index)
            } else {
                None
            }
        })
        .sum();

    let mut list: Vec<_> = list.into_iter().flat_map(|(l, r)| vec![l, r]).collect();
    list.sort_by(|a, b| a.compare(b));

    let div1 = List::from_str("[[2]]").unwrap();
    let ins1 = list
        .binary_search_by(|item| item.compare(&div1))
        .unwrap_err();
    list.insert(ins1, div1);

    let div2 = List::from_str("[[6]]").unwrap();
    let ins2 = list
        .binary_search_by(|item| item.compare(&div2))
        .unwrap_err();
    list.insert(ins2, div2);

    let p2 = (ins1 + 1) * (ins2 + 1);

    (p1, p2)
}

#[test]
fn test() {
    let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(run(input), (13, 140));
}
